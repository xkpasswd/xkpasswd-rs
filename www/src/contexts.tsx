import { ComponentChildren, createContext } from 'preact';
import {
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
  type Dispatch,
  type StateUpdater,
} from 'preact/hooks';
import xkpasswd from './wasm';
import './app.css';

import type * as xktypes from 'src/types/xkpasswd';
import { buildSegmentInputs, type SegmentInputs } from './PasswordBox/segment';

const DEFAULT_WORDS_COUNT = 3;
const DEFAULT_WORD_TRANSFORMS =
  xkpasswd.WordTransform.Lowercase | xkpasswd.WordTransform.Uppercase;
const DEFAULT_SEPARATORS = '.';
const DEFAULT_DIGITS_AFTER = 2;
const DEFAULT_SYMBOLS_AFTER = 2;
const DEFAULT_PADDING_SYMBOLS = '~@$%^&*-_+=:|?/;';
const DEFAULT_ADAPTIVE_COUNT = 32;

type SettingsBuilderType = {
  preset?: xktypes.Preset;
  updatePreset: Dispatch<StateUpdater<xktypes.Preset | undefined>>;
  wordsCount: number;
  updateWordsCount: Dispatch<StateUpdater<number>>;
  wordTransforms: number;
  updateWordTransforms: Dispatch<StateUpdater<number>>;
  separators: string;
  updateSeparators: Dispatch<StateUpdater<string>>;
  digitsBefore: number;
  updateDigitsBefore: Dispatch<StateUpdater<number>>;
  digitsAfter: number;
  updateDigitsAfter: Dispatch<StateUpdater<number>>;
  symbolsBefore: number;
  updateSymbolsBefore: Dispatch<StateUpdater<number>>;
  symbolsAfter: number;
  updateSymbolsAfter: Dispatch<StateUpdater<number>>;
  paddingSymbols: string;
  updatePaddingSymbols: Dispatch<StateUpdater<string>>;
  adaptivePadding: boolean;
  toggleAdaptivePadding: () => void;
  adaptiveCount: number;
  updateAdaptiveCount: Dispatch<StateUpdater<number>>;
};

/** Overrideable fields for live preview — a partial of the inline-typed fields. */
export type PreviewOverrides = Partial<{
  wordsCount: number;
  separators: string;
  digitsBefore: number;
  digitsAfter: number;
  symbolsBefore: number;
  symbolsAfter: number;
  paddingSymbols: string;
  adaptiveCount: number;
}>;

export type UseSettingsType = {
  settings: xktypes.Settings;
  builder: SettingsBuilderType;
  passwd: string;
  entropy: xktypes.Entropy | undefined;
  segmentInputs: SegmentInputs;
  regenerate: () => void;
  regeneratePreview: (overrides: PreviewOverrides) => void;
};

type SettingsContextType = UseSettingsType;

const SettingsContext = createContext<SettingsContextType | undefined>(
  undefined
);

export const useSettings = (): UseSettingsType => {
  const context = useContext(SettingsContext);

  if (!context) {
    throw new Error(`SettingsContext wasn't initialised!`);
  }

  return context;
};

type SettingsProviderProps = {
  children: ComponentChildren;
};

export const SettingsProvider = ({ children }: SettingsProviderProps) => {
  const [settings, updateSettings] = useState<xktypes.Settings>(
    xkpasswd.Settings.fromPreset(xkpasswd.Preset.Default)
  );

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

  // ── Settings rebuild (committed path) ──────────────────────────────────────

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

  // ── Password generation ─────────────────────────────────────────────────────

  const generator = useMemo(() => new xkpasswd.Xkpasswd(), []);

  const [passwd, setPasswd] = useState<string>('');
  const [entropy, setEntropy] = useState<xktypes.Entropy | undefined>(
    undefined
  );
  const [segmentInputs, setSegmentInputs] = useState<SegmentInputs>(
    buildSegmentInputs({
      preset: undefined,
      wordsCount: DEFAULT_WORDS_COUNT,
      wordTransforms: DEFAULT_WORD_TRANSFORMS,
      separators: DEFAULT_SEPARATORS,
      digitsBefore: 0,
      digitsAfter: DEFAULT_DIGITS_AFTER,
      symbolsBefore: 0,
      symbolsAfter: DEFAULT_SYMBOLS_AFTER,
      paddingSymbols: DEFAULT_PADDING_SYMBOLS,
      adaptivePadding: false,
      adaptiveCount: DEFAULT_ADAPTIVE_COUNT,
    })
  );

  /**
   * Track the current builder shape in a ref so effects and callbacks always
   * read the latest committed values without stale closures.
   * Updated synchronously at render time — always current when effects fire.
   */
  const builderShapeRef = useRef({
    preset,
    wordsCount,
    wordTransforms,
    separators,
    digitsBefore,
    digitsAfter,
    symbolsBefore,
    symbolsAfter,
    paddingSymbols,
    adaptivePadding,
    adaptiveCount,
  });
  builderShapeRef.current = {
    preset,
    wordsCount,
    wordTransforms,
    separators,
    digitsBefore,
    digitsAfter,
    symbolsBefore,
    symbolsAfter,
    paddingSymbols,
    adaptivePadding,
    adaptiveCount,
  };

  /**
   * Core generation helper: produce a password from the given Settings.
   * Reads builderShapeRef.current (stable ref, not a dep) for segmentInputs.
   */
  const genFromSettings = useCallback(
    (s: xktypes.Settings) => {
      const result = generator.genPass(s);
      setPasswd(result.passwd);
      setEntropy(result.entropy);
      setSegmentInputs(buildSegmentInputs(builderShapeRef.current));
    },
    [generator]
  );

  /** Committed regen effect: re-generate whenever committed settings change. */
  useEffect(() => {
    genFromSettings(settings);
  }, [settings, genFromSettings]);

  /** Re-generate on demand (run button). */
  const regenerate = useCallback(() => {
    genFromSettings(settings);
  }, [genFromSettings, settings]);

  /**
   * Live preview: generate a password from the committed builder fields with
   * one field overridden, WITHOUT committing to the builder state.
   *
   * This is the caret-safe path: builder state is unchanged, so the editable
   * inputs' `value` props do not change, and their `useEffect([value])` does
   * NOT overwrite `el.value` → the caret survives.
   *
   * Only active in custom mode (preset != null → no-op).
   * All builder fields are listed as deps so the callback always captures
   * the latest committed values and is never stale.
   */
  const regeneratePreview = useCallback(
    (overrides: PreviewOverrides) => {
      if (preset != null) return;

      const effective = {
        wordsCount: overrides.wordsCount ?? wordsCount,
        wordTransforms,
        separators: overrides.separators ?? separators,
        digitsBefore: overrides.digitsBefore ?? digitsBefore,
        digitsAfter: overrides.digitsAfter ?? digitsAfter,
        symbolsBefore: overrides.symbolsBefore ?? symbolsBefore,
        symbolsAfter: overrides.symbolsAfter ?? symbolsAfter,
        paddingSymbols: overrides.paddingSymbols ?? paddingSymbols,
        adaptivePadding,
        adaptiveCount: overrides.adaptiveCount ?? adaptiveCount,
      };

      const newSettings = new xkpasswd.Settings()
        .withWordsCount(effective.wordsCount)
        .withWordTransforms(effective.wordTransforms)
        .withSeparators(effective.separators)
        .withPaddingDigits(effective.digitsBefore, effective.digitsAfter)
        .withPaddingSymbols(effective.paddingSymbols)
        .withPaddingSymbolLengths(
          effective.symbolsBefore,
          effective.symbolsAfter
        );
      const previewSettings = effective.adaptivePadding
        ? newSettings.withAdaptivePadding(effective.adaptiveCount)
        : newSettings.withFixedPadding();

      const { passwd: p, entropy: e } = generator.genPass(previewSettings);
      setPasswd(p);
      setEntropy(e);
      setSegmentInputs(buildSegmentInputs({ preset: undefined, ...effective }));
    },
    [
      generator,
      preset,
      wordsCount,
      wordTransforms,
      separators,
      digitsBefore,
      digitsAfter,
      symbolsBefore,
      symbolsAfter,
      paddingSymbols,
      adaptivePadding,
      adaptiveCount,
    ]
  );

  const builder: SettingsBuilderType = {
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
  };

  return (
    <SettingsContext.Provider
      value={{
        settings,
        builder,
        passwd,
        entropy,
        segmentInputs,
        regenerate,
        regeneratePreview,
      }}
    >
      {children}
    </SettingsContext.Provider>
  );
};
