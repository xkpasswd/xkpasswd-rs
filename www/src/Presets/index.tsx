import { useEffect, useRef, useState } from 'preact/hooks';
import * as xkpasswd from '../../xkpasswd/xkpasswd';
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
  const [visible, setVisible] = useState(false);

  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const self = ref.current;

    if (!self) {
      return;
    }

    const outsideClickListener = (event: MouseEvent) => {
      if (!self.contains(event.target as HTMLElement) && isVisible(self)) {
        setVisible(false);
      }
    };

    document.addEventListener('click', outsideClickListener);

    return () => {
      document.removeEventListener('click', outsideClickListener);
    };
  }, []);

  return (
    <div className="presets-container" ref={ref}>
      <button className="btn" onClick={() => setVisible(!visible)}>
        {title}
      </button>
      {visible && (
        <div
          aria-labelledby="menu-button"
          aria-orientation="vertical"
          className="presets-dropdown"
          role="menu"
          tabIndex={-1}
        >
          {PRESET_OPTIONS.map(({ text, preset }, idx) => (
            <button
              className="preset-option"
              key={`preset_option_${idx}`}
              onClick={() => {
                setTitle(text);
                onSelect(preset);
                setVisible(false);
              }}
            >
              {text}
            </button>
          ))}
        </div>
      )}
    </div>
  );
};

const isVisible = (elem: HTMLElement | null) =>
  !!elem &&
  !!(elem.offsetWidth || elem.offsetHeight || elem.getClientRects().length);

export default Presets;
