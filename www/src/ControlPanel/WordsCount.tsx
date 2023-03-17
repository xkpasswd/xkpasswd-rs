import { useCallback } from 'preact/hooks';
import { pluralize, STRINGIFIED_NUMBERS } from '../utils';
import DropdownButton from '../DropdownButton';
import './styles.css';

const MIN_WORDS_COUNT = 1;
const MAX_WORDS_COUNT = 12;

type Props = {
  value: number;
  onChange: (count: number) => void;
};

const WordsCount = ({ value, onChange }: Props) => {
  const updateWordsCount = useCallback(
    (event: Event) => {
      const target = event.target as HTMLInputElement;
      const wordsCount = Math.min(
        Math.max(parseInt(target.value, 10), MIN_WORDS_COUNT),
        MAX_WORDS_COUNT
      );

      onChange(wordsCount);
    },
    [onChange]
  );

  const suffix = ` ${pluralize(value, 'word')}`;

  return (
    <>
      <DropdownButton
        name="words-count"
        title={STRINGIFIED_NUMBERS[value]}
        buildDropdownClassName={() => 'words-count-dropdown'}
      >
        {() => (
          <input
            className="words-count-slider"
            type="range"
            min={MIN_WORDS_COUNT}
            max={MAX_WORDS_COUNT}
            step={1}
            value={value}
            onChange={updateWordsCount}
          />
        )}
      </DropdownButton>
      {suffix}
    </>
  );
};

export default WordsCount;
