import { useEffect, useState } from 'preact/hooks';
import * as xkpasswd from '../../xkpasswd/xkpasswd';
import Presets from './Presets';
import WordsCount from './WordsCount';
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
        {preset == null && (
          <>
            <WordsCount />
            {'?'}
          </>
        )}
      </span>
    </div>
  );
};

export default ControlPanel;
