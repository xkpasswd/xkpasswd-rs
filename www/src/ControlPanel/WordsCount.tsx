import { useCallback, useEffect, useState } from 'preact/hooks';
import { useSettings } from '../contexts';
import { pluralize, STRINGIFIED_NUMBERS } from '../utils';
import './styles.css';

const DEFAULT_WORDS_COUNT = 3;
const MIN_WORDS_COUNT = 1;
const MAX_WORDS_COUNT = 10;

const WordsCount = () => {
  const { settings, updateSettings } = useSettings();
  const [wordsCount, setWordsCount] = useState(DEFAULT_WORDS_COUNT);
  const [showSlider, setShowSlider] = useState(false);

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
      <button className="btn" onClick={() => setShowSlider(!showSlider)}>
        {STRINGIFIED_NUMBERS[wordsCount]}
      </button>
      {` ${pluralize(wordsCount, 'word')}`}
      {showSlider && (
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
    </>
  );
};

export default WordsCount;
