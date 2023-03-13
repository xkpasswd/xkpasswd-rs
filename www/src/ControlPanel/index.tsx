import { useEffect, useState } from 'preact/hooks';
import * as xkpasswd from '../../xkpasswd/xkpasswd';
import Presets from './Presets';
import Separators from './Separators';
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

  return (
    <div className="section settings">
      <span>
        {'Hey, can you please '}
        <button className="btn btn-generate" onClick={onGenerate}>
          {'generate'}
        </button>
        {' a password using '}
        <Presets preset={preset} onSelect={setPreset} />
        {preset != null ? (
          ' preset?'
        ) : (
          <>
            {' preset, with '}
            <WordsCount value={wordsCount} onChange={setWordsCount} />
            <WordTransforms
              value={wordTransforms}
              onChange={setWordTransforms}
            />
            {', using '}
            <Separators value={separators} onChange={setSeparators} />
            {'?'}
          </>
        )}
      </span>
    </div>
  );
};

export default ControlPanel;
