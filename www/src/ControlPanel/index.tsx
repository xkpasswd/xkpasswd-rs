import { useEffect, useState } from 'preact/hooks';
import * as xkpasswd from '../../xkpasswd/xkpasswd';
import Presets from './Presets';
import WordsCount from './WordsCount';
import { useSettings } from '../contexts';
import './styles.css';

const DEFAULT_WORDS_COUNT = 3;

type Props = {
  onGenerate: () => void;
};

const ControlPanel = ({ onGenerate }: Props) => {
  const { updateSettings } = useSettings();
  const [preset, setPreset] = useState<xkpasswd.Preset | undefined>(
    xkpasswd.Preset.Default
  );
  const [wordsCount, setWordsCount] = useState(DEFAULT_WORDS_COUNT);

  useEffect(() => {
    if (preset != null) {
      updateSettings(xkpasswd.Settings.fromPreset(preset));
      return;
    }

    const settings = new xkpasswd.Settings().withWordsCount(wordsCount);
    updateSettings(settings);
  }, [updateSettings, preset, wordsCount]);

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
            {' preset, using '}
            <WordsCount value={wordsCount} onChange={setWordsCount} />
            {'?'}
          </>
        )}
      </span>
    </div>
  );
};

export default ControlPanel;
