import { useCallback, useEffect, useState } from 'preact/hooks';
import {
  BarsArrowDownIcon,
  BarsArrowUpIcon,
} from '@heroicons/react/24/outline';

import * as xkpasswd from '../../xkpasswd/xkpasswd';
import { useSettings } from '../contexts';

import Presets from './Presets';
import { Separators, PaddingSymbols } from './SymbolsInput';
import {
  WordsCount,
  PaddingDigits,
  PaddingSymbolCounts,
  PaddingStrategy,
} from './CountSlider';
import WordTransforms from './WordTransforms';
import './styles.css';

const DEFAULT_WORDS_COUNT = 3;
const DEFAULT_WORD_TRANSFORMS =
  xkpasswd.WordTransform.Lowercase | xkpasswd.WordTransform.Uppercase;
const DEFAULT_SEPARATORS = '.';
const DEFAULT_DIGITS_AFTER = 2;
const DEFAULT_SYMBOLS_AFTER = 2;
const DEFAULT_PADDING_SYMBOLS = '~@$%^&*-_+=:|?/.;';
const DEFAULT_ADAPTIVE_COUNT = 32;

type Props = {
  onGenerate: () => void;
};

const ControlPanel = ({ onGenerate }: Props) => {
  const { updateSettings } = useSettings();
  const [preset, setPreset] = useState<xkpasswd.Preset | undefined>(undefined);
  const [expanded, setExpanded] = useState(true);
  const [wordsCount, setWordsCount] = useState(DEFAULT_WORDS_COUNT);
  const [wordTransforms, setWordTransforms] = useState(DEFAULT_WORD_TRANSFORMS);
  const [separators, setSeparators] = useState(DEFAULT_SEPARATORS);
  const [digitsBefore, setDigitsBefore] = useState(0);
  const [digitsAfter, setDigitsAfter] = useState(DEFAULT_DIGITS_AFTER);
  const [symbolsBefore, setSymbolsBefore] = useState(0);
  const [symbolsAfter, setSymbolsAfter] = useState(DEFAULT_SYMBOLS_AFTER);
  const [paddingSymbols, setPaddingSymbols] = useState(DEFAULT_PADDING_SYMBOLS);
  const [adaptivePadding, setAdaptivePadding] = useState(false);
  const [adaptiveCount, setAdaptiveCount] = useState(DEFAULT_ADAPTIVE_COUNT);

  useEffect(() => {
    if (preset != null) {
      updateSettings(xkpasswd.Settings.fromPreset(preset));
      return;
    }

    const settings = new xkpasswd.Settings()
      .withWordsCount(wordsCount)
      .withWordTransforms(wordTransforms)
      .withSeparators(separators)
      .withPaddingDigits(digitsBefore, digitsAfter)
      .withPaddingSymbols(paddingSymbols)
      .withPaddingSymbolLengths(symbolsBefore, symbolsAfter);
    const includingPaddingStrategy = adaptivePadding
      ? settings.withAdaptivePadding(adaptiveCount)
      : settings.withFixedPadding();
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

  const toggleExpanded = useCallback(
    () => setExpanded((expanded) => !expanded),
    [setExpanded]
  );

  const toggleAdaptivePadding = useCallback(
    () => setAdaptivePadding((adaptive) => !adaptive),
    [setAdaptivePadding]
  );

  const presetText = preset == null && expanded ? ' preset, with?' : ' preset?';
  const expandConfigs = (
    <ul>
      {[
        <WordsCount
          key="word-count"
          value={wordsCount}
          onChange={setWordsCount}
        />,
        <WordTransforms
          key="word-transforms"
          value={wordTransforms}
          onChange={setWordTransforms}
        />,
        <Separators
          key="separators"
          value={separators}
          onChange={setSeparators}
        />,
        <PaddingDigits
          key="padding-digits"
          before={digitsBefore}
          onChangeBefore={setDigitsBefore}
          after={digitsAfter}
          onChangeAfter={setDigitsAfter}
        />,
        <span key="padding-symbols">
          <PaddingSymbolCounts
            before={symbolsBefore}
            onChangeBefore={setSymbolsBefore}
            after={symbolsAfter}
            onChangeAfter={setSymbolsAfter}
          />
          <PaddingSymbols value={paddingSymbols} onChange={setPaddingSymbols} />
        </span>,
        <PaddingStrategy
          key="padding-strategy"
          adaptive={adaptivePadding}
          onToggleAdaptive={toggleAdaptivePadding}
          adaptiveCount={adaptiveCount}
          onChangeAdaptiveCount={setAdaptiveCount}
        />,
      ].map((element) => (
        <li key={`${element.key}-wrapper`} className="custom-section">
          {element}
        </li>
      ))}
    </ul>
  );

  return (
    <div className="section settings">
      <span>
        {'Hey, can you please '}
        <button className="btn" onClick={onGenerate}>
          {'generate'}
        </button>
        {' a password using '}
        <Presets preset={preset} onSelect={setPreset} />
        {presetText}
        {preset == null && (
          <>
            <button className="btn btn-expand" onClick={toggleExpanded}>
              {expanded ? (
                <BarsArrowUpIcon className="expand-icon" />
              ) : (
                <BarsArrowDownIcon className="expand-icon" />
              )}
            </button>
            {expanded && expandConfigs}
          </>
        )}
      </span>
    </div>
  );
};

export default ControlPanel;
