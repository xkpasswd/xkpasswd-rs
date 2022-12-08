import { useCallback, useEffect, useState } from 'preact/hooks';
import { useSettings } from '../contexts';
import { pluralize, STRINGIFIED_NUMBERS } from '../utils';
import DropdownButton from '../DropdownButton';
import './styles.css';

const DEFAULT_WORDS_COUNT = 3;
const MIN_WORDS_COUNT = 1;
const MAX_WORDS_COUNT = 12;

const WordsCount = () => {
  const { settings, updateSettings } = useSettings();
  const [wordsCount, setWordsCount] = useState(DEFAULT_WORDS_COUNT);

  const updateWordsCount = useCallback(
    (event: Event) => {
      const target = event.target as HTMLInputElement;
      const wordsCount = Math.min(
        Math.max(parseInt(target.value, 10), MIN_WORDS_COUNT),
        MAX_WORDS_COUNT
      );

      setWordsCount(wordsCount);
    },
    [setWordsCount]
  );

  useEffect(() => {
    const newSettings = settings.withWordsCount(wordsCount);
    updateSettings(newSettings);
  }, [wordsCount, updateSettings]); // eslint-disable-line react-hooks/exhaustive-deps

  return (
    <>
      <DropdownButton
        name="words-count"
        title={STRINGIFIED_NUMBERS[wordsCount]}
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
            value={wordsCount}
            onChange={updateWordsCount}
          />
        )}
      </DropdownButton>
      {` ${pluralize(wordsCount, 'word')}`}
    </>
  );
};

export default WordsCount;
