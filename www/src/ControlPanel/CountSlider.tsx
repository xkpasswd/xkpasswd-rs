import { ComponentChildren } from 'preact';
import { useCallback } from 'preact/hooks';
import { pluralize, STRINGIFIED_NUMBERS } from '../utils';
import DropdownButton from '../DropdownButton';
import './styles.css';

const MIN_COUNT = 1;
const MAX_COUNT = 12;

type RenderProps = {
  allowZero?: boolean;
  name: string;
  renderPrefix?: (value: number) => ComponentChildren;
  renderTitle?: (value: number) => ComponentChildren;
  renderSuffix?: (value: number) => ComponentChildren;
};

type Props = {
  value: number;
  onChange: (count: number) => void;
};

type PaddingCountsProps = {
  before: number;
  onChangeBefore: (value: number) => void;
  after: number;
  onChangeAfter: (value: number) => void;
};

const CountSlider = ({
  allowZero = false,
  name,
  value,
  onChange,
  renderPrefix,
  renderTitle,
  renderSuffix,
}: Props & RenderProps) => {
  const minCount = allowZero ? 0 : MIN_COUNT;

  const updateCount = useCallback(
    (event: Event) => {
      const target = event.target as HTMLInputElement;
      const wordsCount = Math.min(
        Math.max(parseInt(target.value, 10), minCount),
        MAX_COUNT
      );

      onChange(wordsCount);
    },
    [onChange, minCount]
  );

  return (
    <>
      {renderPrefix && renderPrefix(value)}
      <DropdownButton
        name={`${name}-count`}
        title={renderTitle && renderTitle(value)}
        buildDropdownClassName={() => 'count-slider-dropdown'}
      >
        {() => (
          <input
            className="count-slider"
            type="range"
            min={minCount}
            max={MAX_COUNT}
            step={1}
            value={value}
            onChange={updateCount}
          />
        )}
      </DropdownButton>
      {renderSuffix && renderSuffix(value)}
    </>
  );
};

export const WordsCount = (props: Props) => (
  <CountSlider
    {...props}
    name="words"
    renderTitle={(value) => STRINGIFIED_NUMBERS[value]}
    renderSuffix={(value) => ` ${pluralize(value, 'word')}`}
  />
);

export const PaddingDigits = ({
  before,
  onChangeBefore,
  after,
  onChangeAfter,
}: PaddingCountsProps) => (
  <span>
    <CountSlider
      allowZero
      name="digits-before"
      renderTitle={(value) => STRINGIFIED_NUMBERS[value]}
      renderSuffix={(value) => ` ${pluralize(value, 'digit')} before`}
      value={before}
      onChange={onChangeBefore}
    />
    {' & '}
    <CountSlider
      allowZero
      name="digits-after"
      renderTitle={(value) => STRINGIFIED_NUMBERS[value]}
      renderSuffix={(value) => ` ${pluralize(value, 'digit')} after`}
      value={after}
      onChange={onChangeAfter}
    />
  </span>
);

export const PaddingSymbolCounts = ({
  before,
  onChangeBefore,
  after,
  onChangeAfter,
}: PaddingCountsProps) => (
  <>
    <CountSlider
      allowZero
      name="symbols-before"
      renderTitle={(value) => STRINGIFIED_NUMBERS[value]}
      renderSuffix={(value) => ` ${pluralize(value, 'symbol')} before`}
      value={before}
      onChange={onChangeBefore}
    />
    {' & '}
    <CountSlider
      allowZero
      name="symbols-after"
      renderTitle={(value) => STRINGIFIED_NUMBERS[value]}
      renderSuffix={(value) => ` ${pluralize(value, 'symbol')} after`}
      value={after}
      onChange={onChangeAfter}
    />
  </>
);
