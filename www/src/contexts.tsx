import { ComponentChildren, createContext } from 'preact';
import {
  useCallback,
  useContext,
  useEffect,
  useState,
  type StateUpdater,
} from 'preact/hooks';
import xkpasswd from './wasm';
import './app.css';

import type * as xktypes from 'src/types/xkpasswd';

const DEFAULT_WORDS_COUNT = 3;
const DEFAULT_WORD_TRANSFORMS =
  xkpasswd.WordTransform.Lowercase | xkpasswd.WordTransform.Uppercase;
const DEFAULT_SEPARATORS = '.';
const DEFAULT_DIGITS_AFTER = 2;
const DEFAULT_SYMBOLS_AFTER = 2;
const DEFAULT_PADDING_SYMBOLS = '~@$%^&*-_+=:|?/.;';
const DEFAULT_ADAPTIVE_COUNT = 32;

type SettingsContextType = {
  settings: xktypes.Settings;
  updateSettings: (settings: xktypes.Settings) => void;
};

type SettingsBuilderType = {
  preset?: xktypes.Preset;
  updatePreset: StateUpdater<xktypes.Preset | undefined>;
  wordsCount: number;
  updateWordsCount: StateUpdater<number>;
  wordTransforms: number;
  updateWordTransforms: StateUpdater<number>;
  separators: string;
  updateSeparators: StateUpdater<string>;
  digitsBefore: number;
  updateDigitsBefore: StateUpdater<number>;
  digitsAfter: number;
  updateDigitsAfter: StateUpdater<number>;
  symbolsBefore: number;
  updateSymbolsBefore: StateUpdater<number>;
  symbolsAfter: number;
  updateSymbolsAfter: StateUpdater<number>;
  paddingSymbols: string;
  updatePaddingSymbols: StateUpdater<string>;
  adaptivePadding: boolean;
  toggleAdaptivePadding: () => void;
  adaptiveCount: number;
  updateAdaptiveCount: StateUpdater<number>;
};

export type UseSettingsType = {
  settings: xktypes.Settings;
  builder: SettingsBuilderType;
};

const SettingsContext = createContext<SettingsContextType | undefined>(
  undefined
);

export const useSettings = (): UseSettingsType => {
  const context = useContext(SettingsContext);

  if (!context) {
    throw new Error(`SettingsContext wasn't initialised!`);
  }

  const { settings, updateSettings } = context;
  const [preset, updatePreset] = useState<number | undefined>(undefined);
  const [wordsCount, updateWordsCount] = useState(DEFAULT_WORDS_COUNT);
  const [wordTransforms, updateWordTransforms] = useState(
    DEFAULT_WORD_TRANSFORMS
  );
  const [separators, updateSeparators] = useState(DEFAULT_SEPARATORS);
  const [digitsBefore, updateDigitsBefore] = useState(0);
  const [digitsAfter, updateDigitsAfter] = useState(DEFAULT_DIGITS_AFTER);
  const [symbolsBefore, updateSymbolsBefore] = useState(0);
  const [symbolsAfter, updateSymbolsAfter] = useState(DEFAULT_SYMBOLS_AFTER);
  const [paddingSymbols, updatePaddingSymbols] = useState(
    DEFAULT_PADDING_SYMBOLS
  );
  const [adaptivePadding, setAdaptivePadding] = useState(false);
  const [adaptiveCount, updateAdaptiveCount] = useState(DEFAULT_ADAPTIVE_COUNT);

  useEffect(() => {
    if (preset != null) {
      updateSettings(xkpasswd.Settings.fromPreset(preset));
      return;
    }

    const newSettings = new xkpasswd.Settings()
      .withWordsCount(wordsCount)
      .withWordTransforms(wordTransforms)
      .withSeparators(separators)
      .withPaddingDigits(digitsBefore, digitsAfter)
      .withPaddingSymbols(paddingSymbols)
      .withPaddingSymbolLengths(symbolsBefore, symbolsAfter);
    const includingPaddingStrategy = adaptivePadding
      ? newSettings.withAdaptivePadding(adaptiveCount)
      : newSettings.withFixedPadding();
    updateSettings(includingPaddingStrategy);
  }, [
    updateSettings,
    preset,
    wordsCount,
    wordTransforms,
    separators,
    digitsBefore,
    digitsAfter,
    paddingSymbols,
    symbolsBefore,
    symbolsAfter,
    adaptivePadding,
    adaptiveCount,
  ]);

  const toggleAdaptivePadding = useCallback(
    () => setAdaptivePadding((adaptive) => !adaptive),
    [setAdaptivePadding]
  );

  return {
    settings,
    builder: {
      preset,
      updatePreset,
      wordsCount,
      updateWordsCount,
      wordTransforms,
      updateWordTransforms,
      separators,
      updateSeparators,
      digitsBefore,
      updateDigitsBefore,
      digitsAfter,
      updateDigitsAfter,
      symbolsBefore,
      updateSymbolsBefore,
      symbolsAfter,
      updateSymbolsAfter,
      paddingSymbols,
      updatePaddingSymbols,
      adaptivePadding,
      toggleAdaptivePadding,
      adaptiveCount,
      updateAdaptiveCount,
    },
  };
};

type SettingsProviderProps = {
  children: ComponentChildren;
};

export const SettingsProvider = ({ children }: SettingsProviderProps) => {
  const [settings, updateSettings] = useState<xktypes.Settings>(
    xkpasswd.Settings.fromPreset(xkpasswd.Preset.Default)
  );

  return (
    <SettingsContext.Provider value={{ settings, updateSettings }}>
      {children}
    </SettingsContext.Provider>
  );
};
