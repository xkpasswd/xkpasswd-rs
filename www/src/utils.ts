export const STRINGIFIED_NUMBERS = [
  'no',
  'one',
  'two',
  'three',
  'four',
  'five',
  'six',
  'seven',
  'eight',
  'nine',
  'ten',
  'eleven',
  'twelve',
];

export const pluralize = (
  amount: number,
  word: string,
  pluralForm?: string
) => {
  if (amount < 2) {
    return word;
  }

  if (pluralForm) {
    return pluralForm;
  }

  return `${word}s`;
};

export const copyToClipboard = (value: string): boolean => {
  const clipboard = navigator.clipboard;

  if (clipboard != null) {
    clipboard.writeText(value);
    return true;
  }

  if (document.execCommand) {
    const el = document.createElement('input');
    el.value = value;
    document.body.append(el);

    el.select();
    el.setSelectionRange(0, value.length);

    if (!document.execCommand('copy')) {
      return false;
    }

    el.remove();
    return true;
  }

  return false;
};
