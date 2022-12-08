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

  return (
    <>
      <DropdownButton
        name="words-count"
        title={STRINGIFIED_NUMBERS[value]}
        buildDropdownClassName={(isRightAlign) =>
          isRightAlign
            ? 'words-count-dropdown right-align'
            : 'words-count-dropdown left-align'
        }
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
      {` ${pluralize(value, 'word')}`}
    </>
  );
};

export default WordsCount;
