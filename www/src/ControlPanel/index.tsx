import { useCallback, useEffect, useState } from 'preact/hooks';
import * as xkpasswd from '../../xkpasswd/xkpasswd';
import Presets from './Presets';
import { useSettings } from '../contexts';
import './styles.css';

type Props = {
  onGenerate: () => void;
};

const ControlPanel = ({ onGenerate }: Props) => {
  const { updateSettings } = useSettings();
  const [preset, setPreset] = useState<xkpasswd.Preset | undefined>(
    xkpasswd.Preset.Default
  );

  useEffect(() => {
    if (preset) {
      updateSettings(xkpasswd.Settings.fromPreset(preset));
    } else {
      updateSettings(new xkpasswd.Settings());
    }
  }, [updateSettings, preset]);

  return (
    <div className="section settings">
      <span>
        {'Hey, can you please '}
        <button className="btn btn-generate" onClick={onGenerate}>
          {'generate'}
        </button>
        {' a password using '}
        <Presets preset={preset} onSelect={setPreset} />
        {preset == null ? ' preset, using ' : ' preset?'}
        {preset == null && <CustomBlock />}
      </span>
    </div>
  );
};

const STRINGIFIED_NUMBERS = [
  'no',
  'one',
  'two',
  'three',
  'four',
  'five',
  'six',
  'seven',
  'eight',
  'nine',
  'ten',
];

const CustomBlock = () => {
  const { settings, updateSettings } = useSettings();
  const [wordsCount, setWordsCount] = useState(3);

  const updateWordsCount = useCallback(() => {
    const newWordsCount = Math.max(
      (wordsCount + 1) % STRINGIFIED_NUMBERS.length,
      3
    );
    setWordsCount(newWordsCount);
    const newSettings = settings.withWordsCount(newWordsCount);
    updateSettings(newSettings);
  }, [wordsCount, setWordsCount, settings, updateSettings]);

  return (
    <>
      <button className="btn" onClick={updateWordsCount}>
        {STRINGIFIED_NUMBERS[wordsCount]}
      </button>
      {` ${pluralize(wordsCount, 'word')}?`}
    </>
  );
};

const pluralize = (amount: number, word: string) => {
  if (amount < 2) {
    return word;
  }

  switch (word) {
    case 'word':
      return 'words';
    default:
      return word;
  }
};

export default ControlPanel;
