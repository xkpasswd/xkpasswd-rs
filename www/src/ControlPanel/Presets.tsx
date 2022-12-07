import { useState } from 'preact/hooks';
import * as xkpasswd from '../../xkpasswd/xkpasswd';
import DropdownButton from '../DropdownButton';
import './styles.css';

const PRESET_OPTIONS = [
  { text: 'Custom', preset: undefined },
  { text: 'Default', preset: xkpasswd.Preset.Default },
  { text: 'Apple ID', preset: xkpasswd.Preset.AppleID },
  { text: 'Windows NTLM v1', preset: xkpasswd.Preset.WindowsNtlmV1 },
  {
    text: 'Security Questions',
    preset: xkpasswd.Preset.SecurityQuestions,
  },
  { text: 'Web16', preset: xkpasswd.Preset.Web16 },
  { text: 'Web32', preset: xkpasswd.Preset.Web32 },
  { text: 'Wifi', preset: xkpasswd.Preset.Wifi },
  { text: 'XKCD', preset: xkpasswd.Preset.Xkcd },
];

type Props = {
  preset?: xkpasswd.Preset;
  onSelect: (preset?: xkpasswd.Preset) => void;
};

const Presets = ({ preset, onSelect }: Props) => {
  const option = PRESET_OPTIONS.find((opt) => preset == opt.preset);
  const [title, setTitle] = useState(option?.text);

  return (
    <DropdownButton
      name="presets"
      title={title}
      dropdownClassName="presets-dropdown"
    >
      {({ dismiss: dismissDropdown }) =>
        PRESET_OPTIONS.map(({ text, preset }, idx) => (
          <button
            className="preset-option"
            key={`preset_option_${idx}`}
            onClick={() => {
              setTitle(text);
              onSelect(preset);
              dismissDropdown();
            }}
          >
            {text}
          </button>
        ))
      }
    </DropdownButton>
  );
};

export default Presets;
