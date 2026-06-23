import { describe, it, expect } from 'vitest';
import {
  segmentPassword,
  buildSegmentInputs,
  type Segment,
  type SegmentInputs,
} from './segment';

// ── helpers ────────────────────────────────────────────────────────────────

/** Core invariant: concatenation of all segment texts must equal the input. */
function assertConcat(passwd: string, segs: Segment[]): void {
  expect(segs.map((s) => s.text).join('')).toBe(passwd);
}

/** Run segmentPassword and immediately assert the concat invariant. */
function seg(passwd: string, inputs: SegmentInputs): Segment[] {
  const segs = segmentPassword(passwd, inputs);
  assertConcat(passwd, segs);
  return segs;
}

/** Minimal fixed-mode inputs (no digits, no symbols, single separator). */
function fixedInputs(overrides: Partial<SegmentInputs> = {}): SegmentInputs {
  return {
    wordsCount: 2,
    separators: '.',
    digitsBefore: 0,
    digitsAfter: 0,
    symbolsBefore: 0,
    symbolsAfter: 0,
    paddingSymbols: '~@$%^&*-_+=:|?/.;',
    adaptivePadding: false,
    adaptiveCount: 0,
    ...overrides,
  };
}

function adaptiveInputs(overrides: Partial<SegmentInputs> = {}): SegmentInputs {
  return {
    wordsCount: 2,
    separators: '.',
    digitsBefore: 0,
    digitsAfter: 0,
    symbolsBefore: 0,
    symbolsAfter: 0,
    paddingSymbols: '~@$%^&*-_+=:|?/.;',
    adaptivePadding: true,
    adaptiveCount: 20,
    ...overrides,
  };
}

// ── edge-case: empty string ────────────────────────────────────────────────

describe('segmentPassword – empty string', () => {
  it('returns [] and satisfies concat invariant', () => {
    const segs = seg('', fixedInputs());
    expect(segs).toHaveLength(0);
  });
});

// ── fixed mode ─────────────────────────────────────────────────────────────

describe('segmentPassword – fixed mode', () => {
  it('single word, no digits/symbols', () => {
    const segs = seg('apple', fixedInputs({ wordsCount: 1 }));
    expect(segs).toEqual([{ text: 'apple', kind: 'word' }]);
  });

  it('two words with separator', () => {
    const segs = seg('apple.banana', fixedInputs());
    expect(segs).toEqual([
      { text: 'apple', kind: 'word' },
      { text: '.', kind: 'sep' },
      { text: 'banana', kind: 'word' },
    ]);
  });

  it('prefix+suffix symbols with digits on both ends', () => {
    const inputs = fixedInputs({
      wordsCount: 2,
      digitsBefore: 2,
      digitsAfter: 2,
      symbolsBefore: 2,
      symbolsAfter: 2,
    });
    const segs = seg('~~12.apple.banana.34~~', inputs);
    expect(segs).toEqual([
      { text: '~~', kind: 'symbol' },
      { text: '12', kind: 'digit' },
      { text: '.', kind: 'sep' },
      { text: 'apple', kind: 'word' },
      { text: '.', kind: 'sep' },
      { text: 'banana', kind: 'word' },
      { text: '.', kind: 'sep' },
      { text: '34', kind: 'digit' },
      { text: '~~', kind: 'symbol' },
    ]);
  });

  it('digitsBefore=1 (single digit)', () => {
    const inputs = fixedInputs({
      wordsCount: 2,
      digitsBefore: 1,
      digitsAfter: 2,
    });
    const segs = seg('1.apple.banana.34', inputs);
    expect(segs).toEqual([
      { text: '1', kind: 'digit' },
      { text: '.', kind: 'sep' },
      { text: 'apple', kind: 'word' },
      { text: '.', kind: 'sep' },
      { text: 'banana', kind: 'word' },
      { text: '.', kind: 'sep' },
      { text: '34', kind: 'digit' },
    ]);
  });

  it('separator char also in symbol pool (. is both)', () => {
    // '.' is in default paddingSymbols '~@$%^&*-_+=:|?/.;'
    const inputs = fixedInputs({
      symbolsBefore: 2,
      symbolsAfter: 2,
    });
    const segs = seg('..apple.banana..', inputs);
    // prefix/suffix '.' chars are homogeneous in paddingSymbols → symbol
    // middle 'apple.banana' detected by '.' separator
    expect(segs).toEqual([
      { text: '..', kind: 'symbol' },
      { text: 'apple', kind: 'word' },
      { text: '.', kind: 'sep' },
      { text: 'banana', kind: 'word' },
      { text: '..', kind: 'symbol' },
    ]);
  });

  it('tilde separator (~ in both separators and paddingSymbols)', () => {
    const inputs = fixedInputs({
      separators: '~-._',
      symbolsBefore: 2,
      symbolsAfter: 2,
    });
    const segs = seg('~~apple~banana~~', inputs);
    expect(segs).toEqual([
      { text: '~~', kind: 'symbol' },
      { text: 'apple', kind: 'word' },
      { text: '~', kind: 'sep' },
      { text: 'banana', kind: 'word' },
      { text: '~~', kind: 'symbol' },
    ]);
  });

  it('all-symbols password triggers char-class fallback', () => {
    const inputs = fixedInputs({ symbolsBefore: 0, symbolsAfter: 0 });
    const segs = seg('~~~~', inputs);
    // no words expected to be found → fallback classifies '~' as symbol
    expect(segs.every((s) => s.kind === 'symbol')).toBe(true);
  });

  it('password shorter than symbol bounds falls back gracefully', () => {
    const inputs = fixedInputs({ symbolsBefore: 3, symbolsAfter: 3 });
    const segs = seg('~~', inputs); // only 2 chars, but bounds sum to 6
    assertConcat('~~', segs);
  });

  it('symbols only on one side (suffix)', () => {
    const inputs = fixedInputs({
      wordsCount: 2,
      symbolsBefore: 0,
      symbolsAfter: 2,
    });
    const segs = seg('apple.banana~~', inputs);
    expect(segs).toEqual([
      { text: 'apple', kind: 'word' },
      { text: '.', kind: 'sep' },
      { text: 'banana', kind: 'word' },
      { text: '~~', kind: 'symbol' },
    ]);
  });

  it('non-homogeneous symbol block falls back to char-class', () => {
    const inputs = fixedInputs({ symbolsBefore: 2, symbolsAfter: 0 });
    const segs = seg('~@apple.banana', inputs); // prefix '~@' is not homogeneous
    assertConcat('~@apple.banana', segs);
  });
});

// ── adaptive mode ──────────────────────────────────────────────────────────

describe('segmentPassword – adaptive mode', () => {
  it('natural password with trailing pad run (tilde)', () => {
    // paddingSymbols has '~', separators has '.' — '~' not in seps ✓
    const inputs = adaptiveInputs({
      paddingSymbols: '~@$%^&*-_+=:|?/.;',
    });
    const segs = seg('apple.banana~~~~~', inputs);
    expect(segs).toEqual([
      { text: 'apple', kind: 'word' },
      { text: '.', kind: 'sep' },
      { text: 'banana', kind: 'word' },
      { text: '~~~~~', kind: 'symbol' },
    ]);
  });

  it('trimmed exactly on word boundary', () => {
    const segs = seg('apple.banana', adaptiveInputs());
    expect(segs).toEqual([
      { text: 'apple', kind: 'word' },
      { text: '.', kind: 'sep' },
      { text: 'banana', kind: 'word' },
    ]);
  });

  it('trimmed mid-word', () => {
    const segs = seg('apple.ban', adaptiveInputs());
    // fallback: should still identify 'apple', '.', 'ban' or use fallback
    assertConcat('apple.ban', segs);
    expect(segs.some((s) => s.kind === 'word')).toBe(true);
  });

  it('adaptive with digits on both ends, unchanged', () => {
    const inputs = adaptiveInputs({
      digitsBefore: 2,
      digitsAfter: 2,
      wordsCount: 2,
    });
    const segs = seg('12.apple.banana.34', inputs);
    expect(segs).toEqual([
      { text: '12', kind: 'digit' },
      { text: '.', kind: 'sep' },
      { text: 'apple', kind: 'word' },
      { text: '.', kind: 'sep' },
      { text: 'banana', kind: 'word' },
      { text: '.', kind: 'sep' },
      { text: '34', kind: 'digit' },
    ]);
  });

  it('adaptive trimmed: digit before preserved, digit after lost', () => {
    const inputs = adaptiveInputs({
      digitsBefore: 2,
      digitsAfter: 2,
      wordsCount: 2,
    });
    // "12.apple.banana.34" trimmed to remove ".34"
    const segs = seg('12.apple.banana', inputs);
    // Should still detect digitsBefore=2 and words
    expect(segs[0]).toEqual({ text: '12', kind: 'digit' });
    assertConcat('12.apple.banana', segs);
  });

  it('adaptive with pad, no words found in natural → concat still correct', () => {
    // edge: natural is empty or all special chars
    const inputs = adaptiveInputs({ paddingSymbols: '~@$%^&*-_+=:|?/.;' });
    const segs = seg('~~~~~', inputs);
    assertConcat('~~~~~', segs);
  });
});

// ── char-class fallback classification ────────────────────────────────────

describe('char-class fallback', () => {
  it('digits run is tagged digit', () => {
    const inputs = fixedInputs({ wordsCount: 0 });
    const segs = seg('1234', inputs);
    assertConcat('1234', segs);
    expect(segs.some((s) => s.kind === 'digit')).toBe(true);
  });

  it('separator-only chars tagged sep when in separators but not symbols', () => {
    // Use a separator not in paddingSymbols
    const inputs = fixedInputs({
      separators: '|',
      paddingSymbols: '~@$%^&*-_+=:?/.;', // no '|'
      wordsCount: 2,
    });
    const segs = seg('apple|banana', inputs);
    expect(segs).toEqual([
      { text: 'apple', kind: 'word' },
      { text: '|', kind: 'sep' },
      { text: 'banana', kind: 'word' },
    ]);
  });

  it('non-alphanumeric char in neither pool classifies as symbol', () => {
    // ',' is not in separators ('.') or default paddingSymbols
    const inputs = fixedInputs({ wordsCount: 0 });
    // ',' is absent from both '.' (separators) and '~@$%^&*-_+=:|?/.;' (paddingSymbols)
    const segs = seg(',', inputs);
    expect(segs).toEqual([{ text: ',', kind: 'symbol' }]);
  });
});

// ── buildSegmentInputs ─────────────────────────────────────────────────────

describe('buildSegmentInputs', () => {
  const builder = {
    preset: undefined,
    wordsCount: 3,
    wordTransforms: 5,
    separators: '-',
    digitsBefore: 1,
    digitsAfter: 2,
    symbolsBefore: 2,
    symbolsAfter: 3,
    paddingSymbols: '~@$%',
    adaptivePadding: false,
    adaptiveCount: 32,
  };

  it('custom builder returns correct inputs', () => {
    const inputs = buildSegmentInputs(builder);
    expect(inputs.wordsCount).toBe(3);
    expect(inputs.separators).toBe('-');
    expect(inputs.digitsBefore).toBe(1);
    expect(inputs.digitsAfter).toBe(2);
    expect(inputs.symbolsBefore).toBe(2);
    expect(inputs.symbolsAfter).toBe(3);
    expect(inputs.paddingSymbols).toBe('~@$%');
    expect(inputs.adaptivePadding).toBe(false);
  });

  it('adaptive=true forces symbolsBefore/After to 0', () => {
    const adaptiveBuilder = { ...builder, adaptivePadding: true };
    const inputs = buildSegmentInputs(adaptiveBuilder);
    expect(inputs.symbolsBefore).toBe(0);
    expect(inputs.symbolsAfter).toBe(0);
    expect(inputs.adaptivePadding).toBe(true);
  });

  it('named preset uses default pools so fallback can color common chars', () => {
    const presetBuilder = { ...builder, preset: 0 };
    const inputs = buildSegmentInputs(presetBuilder, 0);
    // Counts are zeroed → char-class fallback is used
    expect(inputs.wordsCount).toBe(0);
    expect(inputs.digitsBefore).toBe(0);
    expect(inputs.digitsAfter).toBe(0);
    // Default pools are set so common preset chars get colored
    expect(inputs.separators).toBe('.-_~');
    expect(inputs.paddingSymbols).toBe('~@$%^&*-_+=:|?/.;');
    // Concat invariant still holds
    const testPasswd = 'apple.banana';
    assertConcat(testPasswd, segmentPassword(testPasswd, inputs));
  });

  it('named preset mode segments apple-37-banana!@ into multiple colored segments', () => {
    const presetBuilder = { ...builder, preset: 0 };
    const inputs = buildSegmentInputs(presetBuilder, 0);
    const passwd = 'apple-37-banana!@';
    const segs = seg(passwd, inputs);
    // Must produce multiple segments (not a single white 'word' blob)
    expect(segs.length).toBeGreaterThan(1);
    // At least one segment must be non-word (sep, digit, or symbol)
    expect(segs.some((s) => s.kind !== 'word')).toBe(true);
  });
});
