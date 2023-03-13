import { useCallback, useRef } from 'preact/hooks';
import DropdownButton from '../DropdownButton';
import './styles.css';

type Props = {
  value: string;
  onChange: (separators: string) => void;
};

const Separators = ({ value, onChange }: Props) => {
  const separatorsInputRef = useRef<HTMLInputElement>(null);
  const updateSeparators = useCallback(
    (event: Event) => {
      const target = event.target as HTMLInputElement;

      if (target.value.length < 1) return;
      const distinctChars = new Set(target.value.split(''));
      onChange(Array.from(distinctChars).join(''));
    },
    [onChange]
  );

  const suffix = value.length == 1 ? 'separator' : 'separators';

  return (
    <>
      <DropdownButton
        name="separators"
        title={`${value} as ${suffix}`}
        buildDropdownClassName={(isRightAlign) =>
          isRightAlign
            ? 'separators-dropdown right-align'
            : 'separators-dropdown left-align'
        }
        onToggle={(visible) => visible && separatorsInputRef.current?.focus()}
      >
        {() => (
          <input
            autoFocus
            className="separators-input"
            onChange={updateSeparators}
            type="text"
            value={value}
            ref={separatorsInputRef}
          />
        )}
      </DropdownButton>
    </>
  );
};

export default Separators;
