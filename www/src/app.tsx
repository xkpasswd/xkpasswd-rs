import { useCallback, useEffect, useState } from 'preact/hooks';
import * as xkpasswd from '../xkpasswd/xkpasswd';
import Entropy from './Entropy';
import Header from './Header';
import Presets from './Presets';
import './app.css';

const App = () => {
  const [passGenerator] = useState(new xkpasswd.Xkpasswd());
  const [preset, setPreset] = useState<xkpasswd.Preset | undefined>(
    xkpasswd.Preset.Default
  );
  const [settings, setSettings] = useState<xkpasswd.Settings>(
    xkpasswd.Settings.fromPreset(xkpasswd.Preset.Default)
  );
  const [entropy, setEntropy] = useState<xkpasswd.Entropy | undefined>(
    undefined
  );
  const [passwd, setPasswd] = useState<string>('');

  const buildSettings = useCallback(() => {
    let settings =
      preset == null
        ? new xkpasswd.Settings()
        : xkpasswd.Settings.fromPreset(preset);

    setSettings(settings);
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
      <Header />
      <div className="flex items-center break-words">
        <span>
          {'Hey, can you please '}
          <button className="btn btn-generate" onClick={genPasswd}>
            {'generate'}
          </button>
          {' a password using '}
          <Presets preset={preset} onSelect={setPreset} />
          {' preset?'}
        </span>
      </div>
      <span>{`Sure, here you are (just tap to copy):`}</span>
      <div className="passwd" onClick={copyPasswd}>
        {passwd}
      </div>
      <Entropy entropy={entropy} />
    </>
  );
};

export default App;
