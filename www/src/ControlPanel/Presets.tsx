import { useCallback, useEffect, useRef, useState } from 'preact/hooks';
import * as xkpasswd from '../../xkpasswd/xkpasswd';
import { useSettings } from '../contexts';
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

const Presets = () => {
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

  const option = PRESET_OPTIONS.find((opt) => preset === opt.preset);
  const [title, setTitle] = useState(option?.text);
  const [visible, setVisible] = useState(false);
  const [dropdownRightAlign, setDropdownRightAlign] = useState(false);

  const ref = useRef<HTMLDivElement>(null);

  const setDropdownAlignment = useCallback(() => {
    const selfRef = ref.current;
    if (!selfRef) {
      return;
    }

    const { x, width } = selfRef.getBoundingClientRect();
    setDropdownRightAlign(x + width / 2 > window.screen.width / 2);
  }, [setDropdownRightAlign]);

  const toggleDropdown = useCallback(() => {
    if (visible) {
      setVisible(false);
      return;
    }

    setDropdownAlignment();
    setVisible(true);
  }, [visible, setVisible, setDropdownAlignment]);

  useEffect(() => {
    const outsideClickListener = (event: MouseEvent) => {
      if (
        !ref.current?.contains(event.target as HTMLElement) &&
        isVisible(ref.current)
      ) {
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
      <button className="btn" onClick={toggleDropdown}>
        {title}
      </button>
      {visible && (
        <div
          aria-labelledby="menu-button"
          aria-orientation="vertical"
          className={`presets-dropdown ${
            dropdownRightAlign ? 'right-0' : 'left-0'
          }`}
          role="menu"
          tabIndex={-1}
        >
          {PRESET_OPTIONS.map(({ text, preset }, idx) => (
            <button
              className="preset-option"
              key={`preset_option_${idx}`}
              onClick={() => {
                setTitle(text);
                setPreset(preset);
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
