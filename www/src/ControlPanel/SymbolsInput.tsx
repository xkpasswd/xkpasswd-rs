import { useCallback, useRef } from 'preact/hooks';
import DropdownButton from '../DropdownButton';
import './styles.css';

type Props = {
  name: string;
  value: string;
  onChange: (symbols: string) => void;
};

const formatSymbols = (symbols: string): string => symbols.replaceAll(' ', '␣');

const Symbols = ({ name, value, onChange }: Props) => {
  const symbols = useRef<HTMLInputElement>(null);
  const updateSymbols = useCallback(
    (event: Event) => {
      const target = event.target as HTMLInputElement;

      if (target.value.length < 1) return;
      const distinctChars = new Set(target.value.split(''));
      onChange(Array.from(distinctChars).join(''));
    },
    [onChange]
  );

  const namePlural = `${name}s`;
  const prefix = `${formatSymbols(value)} as `;
  const suffix = value.length == 1 ? name : namePlural;

  return (
    <>
      {prefix}
      <DropdownButton
        name={namePlural}
        title={suffix}
        buildDropdownClassName={() => `${namePlural}-dropdown`}
        onToggle={(visible) => visible && symbols.current?.focus()}
      >
        {() => (
          <input
            autoFocus
            className={`${namePlural}-input`}
            onChange={updateSymbols}
            type="text"
            value={value}
            ref={symbols}
          />
        )}
      </DropdownButton>
    </>
  );
};

export default Symbols;
