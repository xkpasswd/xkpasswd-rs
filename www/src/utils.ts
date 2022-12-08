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

export const pluralize = (amount: number, word: string) => {
  if (amount < 2) {
    return word;
  }

  switch (word) {
    case 'word':
      return 'words';
    default:
      return word;
  }
};
