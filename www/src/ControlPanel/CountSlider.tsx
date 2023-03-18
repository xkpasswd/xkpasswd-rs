import { ComponentChildren } from 'preact';
import { useCallback } from 'preact/hooks';
import { pluralize, STRINGIFIED_NUMBERS } from '../utils';
import DropdownButton from '../DropdownButton';
import './styles.css';

const MAX_WORDS_COUNT = 12;
const MAX_PADDING_COUNT = 10;
const MIN_ADAPTIVE_COUNT = 10;
const MAX_ADAPTIVE_COUNT = 64;

type RenderProps = {
  name: string;
  minCount: number;
  maxCount: number;
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

type PaddingStrategyProps = {
  adaptive: boolean;
  onToggleAdaptive: () => void;
  adaptiveCount: number;
  onChangeAdaptiveCount: (value: number) => void;
};

const CountSlider = ({
  value,
  onChange,
  name,
  minCount,
  maxCount,
  renderPrefix,
  renderTitle,
  renderSuffix,
}: Props & RenderProps) => {
  const updateCount = useCallback(
    (event: Event) => {
      const target = event.target as HTMLInputElement;
      const wordsCount = Math.min(
        Math.max(parseInt(target.value, 10), minCount),
        maxCount
      );

      onChange(wordsCount);
    },
    [onChange, minCount, maxCount]
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
            max={maxCount}
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
    minCount={1}
    maxCount={MAX_WORDS_COUNT}
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
      value={before}
      onChange={onChangeBefore}
      name="digits-before"
      minCount={0}
      maxCount={MAX_PADDING_COUNT}
      renderTitle={(value) => STRINGIFIED_NUMBERS[value]}
      renderSuffix={(value) => ` ${pluralize(value, 'digit')} before`}
    />
    {' & '}
    <CountSlider
      value={after}
      onChange={onChangeAfter}
      name="digits-after"
      minCount={0}
      maxCount={MAX_PADDING_COUNT}
      renderTitle={(value) => STRINGIFIED_NUMBERS[value]}
      renderSuffix={(value) => ` ${pluralize(value, 'digit')} after`}
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
      value={before}
      onChange={onChangeBefore}
      name="symbols-before"
      minCount={0}
      maxCount={MAX_PADDING_COUNT}
      renderTitle={(value) => STRINGIFIED_NUMBERS[value]}
      renderSuffix={(value) => ` ${pluralize(value, 'symbol')} before`}
    />
    {' & '}
    <CountSlider
      value={after}
      onChange={onChangeAfter}
      name="symbols-after"
      minCount={0}
      maxCount={MAX_PADDING_COUNT}
      renderTitle={(value) => STRINGIFIED_NUMBERS[value]}
      renderSuffix={(value) => ` ${pluralize(value, 'symbol')} after`}
    />
  </>
);

export const PaddingStrategy = ({
  adaptive,
  onToggleAdaptive,
  adaptiveCount,
  onChangeAdaptiveCount,
}: PaddingStrategyProps) => {
  if (!adaptive) {
    return (
      <button className="btn" onClick={onToggleAdaptive}>
        {'no justifying'}
      </button>
    );
  }

  return (
    <span>
      <button className="btn" onClick={onToggleAdaptive}>
        {'justifying'}
      </button>
      <CountSlider
        value={adaptiveCount}
        onChange={onChangeAdaptiveCount}
        name="adaptive-padding"
        minCount={MIN_ADAPTIVE_COUNT}
        maxCount={MAX_ADAPTIVE_COUNT}
        renderPrefix={() => ' to fit '}
        renderTitle={(value) => `${value} ${pluralize(value, 'character')}`}
      />
    </span>
  );
};
