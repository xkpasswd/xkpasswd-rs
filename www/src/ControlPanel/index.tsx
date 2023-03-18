import { useCallback, useState } from 'preact/hooks';
import {
  BarsArrowDownIcon,
  BarsArrowUpIcon,
} from '@heroicons/react/24/outline';

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

type Props = {
  onGenerate: () => void;
};

const ControlPanel = ({ onGenerate }: Props) => {
  const { builder } = useSettings();
  const [expanded, setExpanded] = useState(true);

  const toggleExpanded = useCallback(
    () => setExpanded((expanded) => !expanded),
    [setExpanded]
  );

  const presetText =
    builder.preset == null && expanded ? ' preset, with?' : ' preset?';
  const ExpandIcon = expanded ? BarsArrowUpIcon : BarsArrowDownIcon;
  const expandConfigs = (
    <ul>
      {[
        <WordsCount
          key="word-count"
          value={builder.wordsCount}
          onChange={builder.updateWordsCount}
        />,
        <WordTransforms
          key="word-transforms"
          value={builder.wordTransforms}
          onChange={builder.updateWordTransforms}
        />,
        <Separators
          key="separators"
          value={builder.separators}
          onChange={builder.updateSeparators}
        />,
        <PaddingDigits
          key="padding-digits"
          before={builder.digitsBefore}
          onChangeBefore={builder.updateDigitsBefore}
          after={builder.digitsAfter}
          onChangeAfter={builder.updateDigitsAfter}
        />,
        <span key="padding-symbols">
          <PaddingSymbolCounts
            before={builder.symbolsBefore}
            onChangeBefore={builder.updateSymbolsBefore}
            after={builder.symbolsAfter}
            onChangeAfter={builder.updateSymbolsAfter}
          />
          <PaddingSymbols
            value={builder.paddingSymbols}
            onChange={builder.updatePaddingSymbols}
          />
        </span>,
        <PaddingStrategy
          key="padding-strategy"
          adaptive={builder.adaptivePadding}
          onToggleAdaptive={builder.toggleAdaptivePadding}
          adaptiveCount={builder.adaptiveCount}
          onChangeAdaptiveCount={builder.updateAdaptiveCount}
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
        <Presets preset={builder.preset} onSelect={builder.updatePreset} />
        {presetText}
        {builder.preset == null && (
          <>
            <button className="btn btn-expand" onClick={toggleExpanded}>
              <ExpandIcon className="expand-icon" />
            </button>
            {expanded && expandConfigs}
          </>
        )}
      </span>
    </div>
  );
};

export default ControlPanel;
