import { useCallback, useEffect, useState } from 'preact/hooks';
import * as xkpasswd from '../../xkpasswd/xkpasswd';
import Presets from './Presets';
import SymbolsInput from './SymbolsInput';
import WordsCount from './WordsCount';
import WordTransforms from './WordTransforms';
import { useSettings } from '../contexts';
import './styles.css';

const DEFAULT_WORDS_COUNT = 3;
const DEFAULT_WORD_TRANSFORMS =
  xkpasswd.WordTransform.Lowercase | xkpasswd.WordTransform.Uppercase;
const DEFAULT_SEPARATORS = '.';

type Props = {
  onGenerate: () => void;
};

const ControlPanel = ({ onGenerate }: Props) => {
  const { updateSettings } = useSettings();
  const [preset, setPreset] = useState<xkpasswd.Preset | undefined>(undefined);
  const [expanded, setExpanded] = useState(false);
  const [wordsCount, setWordsCount] = useState(DEFAULT_WORDS_COUNT);
  const [wordTransforms, setWordTransforms] = useState(DEFAULT_WORD_TRANSFORMS);
  const [separators, setSeparators] = useState(DEFAULT_SEPARATORS);

  useEffect(() => {
    if (preset != null) {
      updateSettings(xkpasswd.Settings.fromPreset(preset));
      return;
    }

    const settings = new xkpasswd.Settings()
      .withWordsCount(wordsCount)
      .withWordTransforms(wordTransforms)
      .withSeparators(separators);
    updateSettings(settings);
  }, [updateSettings, preset, wordsCount, wordTransforms, separators]);

  const onExpand = useCallback(
    () => setExpanded((expanded) => !expanded),
    [setExpanded]
  );

  const presetText = preset == null && expanded ? ' preset, with?' : ' preset?';
  const expandArrow = expanded ? '⇱ ' : '⇲ ';

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
        <button className="btn btn-expand" onClick={onExpand}>
          {expandArrow}
        </button>
        {preset == null && expanded && (
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
              <SymbolsInput
                key="separators"
                name="separator"
                value={separators}
                onChange={setSeparators}
              />,
            ].map((element) => (
              <li key={`${element.key}-wrapper`} className="custom-section">
                {element}
              </li>
            ))}
          </ul>
        )}
      </span>
    </div>
  );
};

export default ControlPanel;
