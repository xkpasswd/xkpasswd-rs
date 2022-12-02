import { useCallback, useEffect, useState } from 'preact/hooks';
import * as xkpasswd from '../xkpasswd/xkpasswd';
import {
  ArrowPathIcon,
  ClipboardDocumentIcon,
} from '@heroicons/react/20/solid';
import './app.css';

const PRESET_OPTIONS = [
  { text: 'Default', preset: xkpasswd.Preset.Default },
  { text: 'Apple ID', preset: xkpasswd.Preset.AppleID },
  { text: 'Windows NTLM v1', preset: xkpasswd.Preset.WindowsNtlmV1 },
  { text: 'Security Questions', preset: xkpasswd.Preset.SecurityQuestions },
  { text: 'Web16', preset: xkpasswd.Preset.Web16 },
  { text: 'Web32', preset: xkpasswd.Preset.Web32 },
  { text: 'Wifi', preset: xkpasswd.Preset.Wifi },
  { text: 'XKCD', preset: xkpasswd.Preset.Xkcd },
];

export function App() {
  const [passGenerator] = useState(new xkpasswd.Xkpasswd());
  const [settings, setSettings] = useState<xkpasswd.Settings | undefined>(
    undefined
  );
  const [preset, setPreset] = useState<xkpasswd.Preset>(
    xkpasswd.Preset.Default
  );
  const [passwd, setPasswd] = useState<string>('');

  const buildSettings = useCallback(() => {
    setSettings(xkpasswd.Settings.fromPreset(preset));
  }, [preset]);

  const genPasswd = useCallback(
    () => settings && setPasswd(passGenerator.genPass(settings)),
    [passGenerator, settings]
  );

  useEffect(buildSettings, [buildSettings, preset]);
  useEffect(genPasswd, [genPasswd, passGenerator, settings]);

  return (
    <>
      <div>
        <select
          className="text-lg text-center mx-auto mb-2 py-2 w-64"
          onChange={(event) => {
            const target = event.target as HTMLSelectElement;
            const idx = parseInt(target.value, 10);
            setPreset(PRESET_OPTIONS[idx].preset);
          }}
          name="presets"
          value={preset}
        >
          {PRESET_OPTIONS.map(({ text }, idx) => (
            <option key={`preset_option_${idx}`} value={idx}>
              {text}
            </option>
          ))}
        </select>
      </div>
      <div className="mt-2">
        <button className="btn btn-generate" onClick={genPasswd}>
          <ArrowPathIcon class="h-6" />
        </button>
        <button
          className="btn btn-copy"
          onClick={() => navigator.clipboard.writeText(passwd)}
        >
          <ClipboardDocumentIcon class="h-6" />
        </button>
      </div>
      <div className="bg-teal-700 sm:bg-transparent mt-3 py-2">
        <span className="sm:bg-teal-700 p-3 font-bold font-mono text-white mx-auto break-words">
          {passwd}
        </span>
      </div>
    </>
  );
}
