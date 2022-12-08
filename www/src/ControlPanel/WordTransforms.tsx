import { useCallback, useEffect, useState } from 'preact/hooks';
import * as xkpasswd from '../../xkpasswd/xkpasswd';
import './styles.css';
import DropdownButton from '../DropdownButton';

const SINGLE_WORD_TRANSFORMS = [
  {
    name: 'lowercase',
    text: 'lowercase',
    value: xkpasswd.WordTransform.Lowercase,
  },
  {
    name: 'titlecase',
    text: 'Titlecase',
    value: xkpasswd.WordTransform.Titlecase,
  },
  {
    name: 'uppercase',
    text: 'UPPERCASE',
    value: xkpasswd.WordTransform.Uppercase,
  },
  {
    name: 'inversed-titlecase',
    text: 'iNVERSED TITLECASE',
    value: xkpasswd.WordTransform.InversedTitlecase,
  },
];

const GROUP_WORD_TRANSFORMS = [
  {
    name: 'altercase-lower-first',
    text: 'altercase LOWER first',
    value: xkpasswd.WordTransform.AltercaseLowerFirst,
  },
  {
    name: 'altercase-upper-first',
    text: 'ALTERCASE upper FIRST',
    value: xkpasswd.WordTransform.AltercaseUpperFirst,
  },
];

type Props = {
  value: number;
  onChange: (transforms: number) => void;
};

const WordTransforms = ({ value, onChange }: Props) => {
  const [transforms, setTransforms] = useState<Set<xkpasswd.WordTransform>>(
    parseTransforms(value)
  );
  const [oneSingleTransformSelected, setOneSingleTransformSelected] =
    useState(false);

  useEffect(() => {
    setOneSingleTransformSelected(transforms.size == 1);
    onChange(Array.from(transforms).reduce((acc, cur) => acc | cur, 0));
  }, [onChange, transforms]);

  const onToggleSingleTransform = useCallback(
    (event: Event) => {
      const target = event.target as HTMLInputElement;
      const toggledTransform = parseInt(target.value, 10);

      const newTransforms = new Set(transforms);
      newTransforms.delete(xkpasswd.WordTransform.AltercaseLowerFirst);
      newTransforms.delete(xkpasswd.WordTransform.AltercaseUpperFirst);

      if (target.checked) {
        newTransforms.add(toggledTransform);
      } else {
        newTransforms.delete(toggledTransform);
      }

      setTransforms(newTransforms);
    },
    [transforms]
  );

  const onSelectGroupTransform = useCallback((event: Event) => {
    const target = event.target as HTMLInputElement;
    const selectedTransform = parseInt(target.value, 10);
    setTransforms(new Set([selectedTransform]));
  }, []);

  const prefix = transforms.size == 1 ? ' of ' : ' mixing of ';
  const suffix = transforms.size == 1 ? ' only' : '';

  return (
    <>
      {prefix}
      <DropdownButton
        name="word-transforms"
        title={buildTransformsText(transforms)}
        buildDropdownClassName={() => 'p-2 w-56'}
      >
        {() => (
          <>
            {SINGLE_WORD_TRANSFORMS.map(({ name, text, value }) => (
              <span key={`word-transform-${name}`}>
                <input
                  type="checkbox"
                  id={`word-transform-${name}`}
                  value={value}
                  checked={transforms.has(value)}
                  onChange={onToggleSingleTransform}
                  disabled={transforms.has(value) && oneSingleTransformSelected}
                />
                <label
                  className="select-none ml-1"
                  for={`word-transform-${name}`}
                >
                  {text}
                </label>
              </span>
            ))}
            {GROUP_WORD_TRANSFORMS.map(({ name, text, value }) => (
              <span key={`word-transform-${name}`}>
                <input
                  type="radio"
                  id={`word-transform-${name}`}
                  name="word-transforms"
                  value={value}
                  checked={transforms.has(value)}
                  onChange={onSelectGroupTransform}
                />
                <label
                  className="select-none ml-1"
                  for={`word-transform-${name}`}
                >
                  {text}
                </label>
              </span>
            ))}
          </>
        )}
      </DropdownButton>
      {suffix}
    </>
  );
};

const parseTransforms = (transforms: number) => {
  if ((transforms & xkpasswd.WordTransform.AltercaseLowerFirst) > 0) {
    return new Set([xkpasswd.WordTransform.AltercaseLowerFirst]);
  }

  if ((transforms & xkpasswd.WordTransform.AltercaseUpperFirst) > 0) {
    return new Set([xkpasswd.WordTransform.AltercaseUpperFirst]);
  }

  return new Set(
    SINGLE_WORD_TRANSFORMS.map(({ value }) => value).filter(
      (flag) => (transforms & flag) > 0
    )
  );
};

const buildTransformsText = (transforms: Set<xkpasswd.WordTransform>) => {
  const texts = [...GROUP_WORD_TRANSFORMS, ...SINGLE_WORD_TRANSFORMS]
    .filter(({ value }) => transforms.has(value))
    .map(({ text }) => text);

  if (texts.length < 2) {
    return texts.join('');
  }

  return 'multiple transforms';
};

export default WordTransforms;
