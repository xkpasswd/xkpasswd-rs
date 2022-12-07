import { ComponentChildren, createContext } from 'preact';
import { useContext, useState } from 'preact/hooks';
import * as xkpasswd from '../xkpasswd/xkpasswd';
import './app.css';

type SettingsContextType = {
  settings: xkpasswd.Settings;
  updateSettings: (settings: xkpasswd.Settings) => void;
};

const SettingsContext = createContext<SettingsContextType | undefined>(
  undefined
);

export const useSettings = () => {
  const context = useContext(SettingsContext);

  if (!context) {
    throw new Error(`SettingsContext wasn't initialised!`);
  }

  return context;
};

type SettingsProviderProps = {
  children: ComponentChildren;
};

export const SettingsProvider = ({ children }: SettingsProviderProps) => {
  const [settings, updateSettings] = useState<xkpasswd.Settings>(
    xkpasswd.Settings.fromPreset(xkpasswd.Preset.Default)
  );

  return (
    <SettingsContext.Provider value={{ settings, updateSettings }}>
      {children}
    </SettingsContext.Provider>
  );
};
