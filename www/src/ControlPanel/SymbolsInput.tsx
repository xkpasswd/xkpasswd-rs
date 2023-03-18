import { ComponentChildren } from 'preact';
import { useCallback, useRef } from 'preact/hooks';
import DropdownButton from '../DropdownButton';
import './styles.css';

type RenderProps = {
  name: string;
  renderPrefix?: (formattedSymbols: string) => ComponentChildren;
  renderTitle?: (formattedSymbols: string) => ComponentChildren;
  renderSuffix?: (formattedSymbols: string) => ComponentChildren;
};

type Props = {
  value: string;
  onChange: (symbols: string) => void;
};

const formatSymbols = (symbols: string): string => symbols.replaceAll(' ', '␣');

const SymbolsInput = ({
  name,
  value,
  onChange,
  renderPrefix,
  renderTitle,
  renderSuffix,
}: Props & RenderProps) => {
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

  const formattedSymbols = formatSymbols(value);

  return (
    <>
      {renderPrefix && renderPrefix(formattedSymbols)}
      <DropdownButton
        name={name}
        title={renderTitle && renderTitle(formattedSymbols)}
        buildDropdownClassName={() => `${name}-dropdown`}
        onToggle={(visible) => visible && symbols.current?.focus()}
      >
        {() => (
          <input
            autoFocus
            className={`${name}-input`}
            onChange={updateSymbols}
            type="text"
            value={value}
            ref={symbols}
          />
        )}
      </DropdownButton>
      {renderSuffix && renderSuffix(formattedSymbols)}
    </>
  );
};

export const Separators = (props: Props) => (
  <SymbolsInput
    {...props}
    name="separators"
    renderPrefix={(symbols) => (
      <>
        {symbols.length > 1 ? 'one of ' : 'only '}
        <span className="formatted-symbols">{symbols}</span>
        {' as '}
      </>
    )}
    renderTitle={(symbols) => (symbols.length > 1 ? 'separators' : 'separator')}
  />
);

export const PaddingSymbols = (props: Props) => (
  <SymbolsInput
    {...props}
    name="padding-symbols"
    renderPrefix={() => ' '}
    renderTitle={() => 'using'}
    renderSuffix={(symbols) => (
      <>
        {symbols.length > 1 ? ' one of ' : ' only '}
        <span className="formatted-symbols">{symbols}</span>
      </>
    )}
  />
);
