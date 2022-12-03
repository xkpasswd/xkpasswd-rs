import * as xkpasswd from '../../xkpasswd/xkpasswd';
import './styles.css';

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

type Props = {
  preset: xkpasswd.Preset;
  onSelectPreset: (preset: xkpasswd.Preset) => void;
};

const Presets = ({ preset, onSelectPreset }: Props) => (
  <select
    className="presets"
    onChange={(event) => {
      const target = event.target as HTMLSelectElement;
      const idx = parseInt(target.value, 10);
      onSelectPreset(PRESET_OPTIONS[idx].preset);
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
);

export default Presets;
