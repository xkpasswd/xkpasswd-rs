import { useCallback, useEffect, useState } from 'preact/hooks';
import * as xkpasswd from '../xkpasswd/xkpasswd';
import { ArrowPathIcon } from '@heroicons/react/20/solid';
import Entropy from './Entropy';
import Presets from './Presets';
import './app.css';

export function App() {
  const [passGenerator] = useState(new xkpasswd.Xkpasswd());
  const [preset, setPreset] = useState<xkpasswd.Preset>(
    xkpasswd.Preset.Default
  );
  const [settings, setSettings] = useState<xkpasswd.Settings>(
    xkpasswd.Settings.fromPreset(preset)
  );
  const [entropy, setEntropy] = useState<xkpasswd.Entropy | undefined>(
    undefined
  );
  const [passwd, setPasswd] = useState<string>('');

  const buildSettings = useCallback(() => {
    setSettings(xkpasswd.Settings.fromPreset(preset));
  }, [preset]);

  const genPasswd = useCallback(() => {
    const { passwd, entropy } = passGenerator.genPass(settings);
    setPasswd(passwd);
    setEntropy(entropy);
  }, [passGenerator, settings]);

  const copyPasswd = useCallback(
    () => navigator.clipboard.writeText(passwd),
    [passwd]
  );

  useEffect(buildSettings, [buildSettings, preset]);
  useEffect(genPasswd, [genPasswd, passGenerator, settings]);

  return (
    <>
      <div className="">
        <Presets preset={preset} onSelectPreset={setPreset} />
        <button className="btn btn-generate" onClick={genPasswd}>
          <ArrowPathIcon class="h-6" />
        </button>
      </div>
      <div className="passwd-container" onClick={copyPasswd}>
        <span className="passwd">{passwd}</span>
      </div>
      <Entropy entropy={entropy} />
    </>
  );
}
