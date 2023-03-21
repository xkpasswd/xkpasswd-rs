import { useCallback, useEffect, useState } from 'preact/hooks';
import ControlPanel from './ControlPanel';
import Entropy from './Entropy';
import Header from './Header';
import PasswordBox from './PasswordBox';
import Version from './Version';
import { SettingsProvider, useSettings } from './contexts';
import xkpasswd from './wasm';
import './app.css';

import type * as xktypes from 'src/types/xkpasswd';

const App = () => {
  const { settings } = useSettings();
  const [passGenerator] = useState(new xkpasswd.Xkpasswd());
  const [entropy, setEntropy] = useState<xktypes.Entropy | undefined>(
    undefined
  );
  const [passwd, setPasswd] = useState<string>('');

  const genPasswd = useCallback(() => {
    const { passwd, entropy } = passGenerator.genPass(settings);
    setPasswd(passwd);
    setEntropy(entropy);
  }, [passGenerator, settings]);

  useEffect(genPasswd, [genPasswd, passGenerator, settings]);

  return (
    <>
      <Header />
      <ControlPanel onGenerate={genPasswd} />
      <PasswordBox passwd={passwd} />
      <Entropy entropy={entropy} />
      <Version />
    </>
  );
};

const AppWrapper = () => (
  <SettingsProvider>
    <App />
  </SettingsProvider>
);

export default AppWrapper;
