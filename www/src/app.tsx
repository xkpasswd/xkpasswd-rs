import ControlPanel from './ControlPanel';
import Entropy from './Entropy';
import Header from './Header';
import PasswordBox from './PasswordBox';
import Version from './Version';
import { SettingsProvider, useSettings } from './contexts';
import './app.css';

const App = () => {
  const { passwd, entropy, regenerate } = useSettings();

  return (
    <>
      <Header />
      {/* .body provides padding for content while the titlebar stays flush */}
      <div className="body">
        <ControlPanel onGenerate={regenerate} />
        <PasswordBox passwd={passwd} />
        <Entropy entropy={entropy} />
        <Version />
      </div>
    </>
  );
};

const AppWrapper = () => (
  <SettingsProvider>
    <App />
  </SettingsProvider>
);

export default AppWrapper;
