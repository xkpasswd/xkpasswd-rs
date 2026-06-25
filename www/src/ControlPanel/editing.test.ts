import { describe, it, expect } from 'vitest';
import {
  FIELD,
  clampNumber,
  normalizeNumber,
  normalizePool,
  activeAltercase,
  canonicalTransforms,
  selectedCases,
  toggleCase,
  toggleAltercase,
} from './editing';

// ── FIELD constants ──────────────────────────────────────────────────────────

describe('FIELD constants', () => {
  it('has all required keys', () => {
    const keys = [
      'wordsCount',
      'digitsBefore',
      'digitsAfter',
      'symbolsBefore',
      'symbolsAfter',
      'adaptiveCount',
    ] as const;
    for (const key of keys) {
      expect(FIELD[key]).toBeDefined();
      expect(typeof FIELD[key].min).toBe('number');
      expect(typeof FIELD[key].max).toBe('number');
      expect(typeof FIELD[key].default).toBe('number');
    }
  });

  it('wordsCount: min=1, max=10, default=3', () => {
    expect(FIELD.wordsCount).toEqual({ min: 1, max: 10, default: 3 });
  });

  it('digitsBefore: min=0, max=10, default=0', () => {
    expect(FIELD.digitsBefore).toEqual({ min: 0, max: 10, default: 0 });
  });

  it('digitsAfter: min=0, max=10, default=2', () => {
    expect(FIELD.digitsAfter).toEqual({ min: 0, max: 10, default: 2 });
  });

  it('symbolsBefore: min=0, max=10, default=0', () => {
    expect(FIELD.symbolsBefore).toEqual({ min: 0, max: 10, default: 0 });
  });

  it('symbolsAfter: min=0, max=10, default=2', () => {
    expect(FIELD.symbolsAfter).toEqual({ min: 0, max: 10, default: 2 });
  });

  it('adaptiveCount: min=4, max=64, default=32 (engine errors on 0)', () => {
    expect(FIELD.adaptiveCount).toEqual({ min: 4, max: 64, default: 32 });
  });
});

// ── clampNumber ──────────────────────────────────────────────────────────────

describe('clampNumber', () => {
  it('empty string → min (for live typing)', () => {
    expect(clampNumber('', 0, 10)).toBe(0);
    expect(clampNumber('', 1, 10)).toBe(1);
    expect(clampNumber('', 4, 64)).toBe(4); // adaptive min
  });

  it('non-numeric string → min', () => {
    expect(clampNumber('abc', 0, 10)).toBe(0);
    expect(clampNumber('xyz', 1, 10)).toBe(1);
  });

  it('strips non-digit chars and parses remaining digits', () => {
    expect(clampNumber('5abc', 0, 10)).toBe(5);
    expect(clampNumber('abc5', 0, 10)).toBe(5);
    // digits extracted = "12", then clamped to max=10
    expect(clampNumber('1a2', 0, 10)).toBe(10);
  });

  it('value within range → exact value', () => {
    expect(clampNumber('5', 0, 10)).toBe(5);
    expect(clampNumber('3', 1, 10)).toBe(3);
  });

  it('value above max → max', () => {
    expect(clampNumber('99', 0, 10)).toBe(10);
    expect(clampNumber('100', 4, 64)).toBe(64);
  });

  it('value below min → min', () => {
    expect(clampNumber('0', 1, 10)).toBe(1);
    expect(clampNumber('3', 4, 64)).toBe(4);
  });

  it('value at min → min', () => {
    expect(clampNumber('0', 0, 10)).toBe(0);
    expect(clampNumber('4', 4, 64)).toBe(4);
  });

  it('value at max → max', () => {
    expect(clampNumber('10', 0, 10)).toBe(10);
    expect(clampNumber('64', 4, 64)).toBe(64);
  });
});

// ── normalizeNumber ──────────────────────────────────────────────────────────

describe('normalizeNumber', () => {
  it('empty string → fallbackDefault (on blur)', () => {
    expect(normalizeNumber('', 0, 10, 2)).toBe(2);
    expect(normalizeNumber('', 1, 10, 3)).toBe(3);
    expect(normalizeNumber('', 4, 64, 32)).toBe(32); // adaptiveCount default
  });

  it('non-numeric (no digits) → fallbackDefault (same as empty string on blur)', () => {
    // 'abc' strips to '' → fallbackDefault applies, NOT min
    expect(normalizeNumber('abc', 0, 10, 5)).toBe(5);
    expect(normalizeNumber('xyz', 1, 10, 3)).toBe(3);
  });

  it('valid value within range → clamped value', () => {
    expect(normalizeNumber('5', 0, 10, 0)).toBe(5);
    expect(normalizeNumber('32', 4, 64, 32)).toBe(32);
  });

  it('value above max → max', () => {
    expect(normalizeNumber('99', 0, 10, 0)).toBe(10);
  });

  it('value below min → min', () => {
    expect(normalizeNumber('0', 1, 10, 3)).toBe(1);
  });

  it('adaptiveCount empty → default 32', () => {
    const { min, max, default: def } = FIELD.adaptiveCount;
    expect(normalizeNumber('', min, max, def)).toBe(32);
  });
});

// ── normalizePool ────────────────────────────────────────────────────────────

describe('normalizePool', () => {
  it('empty string → fallback', () => {
    expect(normalizePool('', '.')).toBe('.');
    expect(normalizePool('', '%')).toBe('%');
  });

  it('whitespace-only → fallback (after trim)', () => {
    expect(normalizePool('   ', '.')).toBe('.');
    expect(normalizePool('\t\n', '%')).toBe('%');
  });

  it('normal string → trimmed', () => {
    expect(normalizePool('  abc  ', '.')).toBe('abc');
    expect(normalizePool('!@#', '.')).toBe('!@#');
  });

  it('already trimmed → unchanged', () => {
    expect(normalizePool('abc', '.')).toBe('abc');
  });

  it('no deduplication (raw chars preserved)', () => {
    expect(normalizePool('aab', '.')).toBe('aab');
    expect(normalizePool('!!', '.')).toBe('!!');
  });
});

// ── activeAltercase ───────────────────────────────────────────────────────────

describe('activeAltercase', () => {
  it('0 → 0 (no transforms)', () => {
    expect(activeAltercase(0)).toBe(0);
  });

  it('64 → 64 (altercase-lower-first)', () => {
    expect(activeAltercase(64)).toBe(64);
  });

  it('128 → 128 (altercase-upper-first)', () => {
    expect(activeAltercase(128)).toBe(128);
  });

  it('64|128 → 64 (64 wins when both bits set, mirrors engine)', () => {
    expect(activeAltercase(64 | 128)).toBe(64);
  });

  it('1|4 → 0 (case-only bits, no altercase)', () => {
    expect(activeAltercase(1 | 4)).toBe(0);
  });

  it('1|4|64 → 64 (altercase bit present alongside case bits)', () => {
    expect(activeAltercase(1 | 4 | 64)).toBe(64);
  });
});

// ── canonicalTransforms ───────────────────────────────────────────────────────

describe('canonicalTransforms', () => {
  it('5 (1|4=lowercase|uppercase) → [1, 4] in display order', () => {
    expect(canonicalTransforms(5)).toEqual([1, 4]);
  });

  it('1|2|4|8 → [1, 2, 4, 8] (all cases in order)', () => {
    expect(canonicalTransforms(1 | 2 | 4 | 8)).toEqual([1, 2, 4, 8]);
  });

  it('64 → [64] (altercase-lower-first only)', () => {
    expect(canonicalTransforms(64)).toEqual([64]);
  });

  it('128 → [128] (altercase-upper-first only)', () => {
    expect(canonicalTransforms(128)).toEqual([128]);
  });

  it('1|4|64 → [64] (altercase wins; case bits suppressed)', () => {
    expect(canonicalTransforms(1 | 4 | 64)).toEqual([64]);
  });

  it('bitfield input order is irrelevant → canonical order always out', () => {
    // 8|1 = 9, same as 1|8
    expect(canonicalTransforms(8 | 1)).toEqual([1, 8]);
    expect(canonicalTransforms(4 | 2 | 1)).toEqual([1, 2, 4]);
  });

  it('2|8 → [2, 8] (titlecase + inversed-titlecase)', () => {
    expect(canonicalTransforms(2 | 8)).toEqual([2, 8]);
  });
});

// ── selectedCases ─────────────────────────────────────────────────────────────

describe('selectedCases', () => {
  it('0 → [] (no bits set)', () => {
    expect(selectedCases(0)).toEqual([]);
  });

  it('1 → [1] (lowercase only)', () => {
    expect(selectedCases(1)).toEqual([1]);
  });

  it('5 (1|4) → [1, 4] (lowercase + uppercase)', () => {
    expect(selectedCases(5)).toEqual([1, 4]);
  });

  it('64 → [] (altercase bits ignored)', () => {
    expect(selectedCases(64)).toEqual([]);
  });

  it('128 → [] (altercase bits ignored)', () => {
    expect(selectedCases(128)).toEqual([]);
  });

  it('1|4|64 → [1, 4] (ignores altercase bit)', () => {
    expect(selectedCases(1 | 4 | 64)).toEqual([1, 4]);
  });

  it('1|2|4|8 → [1, 2, 4, 8] (all cases)', () => {
    expect(selectedCases(1 | 2 | 4 | 8)).toEqual([1, 2, 4, 8]);
  });
});

// ── toggleCase ────────────────────────────────────────────────────────────────

describe('toggleCase', () => {
  // Case mode: add a missing case bit
  it('case mode: adds a missing case bit', () => {
    expect(toggleCase(1, 4)).toBe(1 | 4);  // add uppercase
    expect(toggleCase(4, 2)).toBe(4 | 2);  // add titlecase
  });

  // Case mode: remove a present case bit (not sole)
  it('case mode: removes a present case bit when not sole', () => {
    expect(toggleCase(1 | 4, 4)).toBe(1);  // remove uppercase
    expect(toggleCase(1 | 4, 1)).toBe(4);  // remove lowercase
    expect(toggleCase(1 | 2 | 4, 2)).toBe(1 | 4); // remove middle
  });

  // Case mode: lock-last — sole remaining case is a no-op
  it('case mode: lock-last — cannot remove sole remaining case', () => {
    expect(toggleCase(1, 1)).toBe(1);   // only lowercase, cannot remove
    expect(toggleCase(4, 4)).toBe(4);   // only uppercase, cannot remove
    expect(toggleCase(8, 8)).toBe(8);   // only inversed-titlecase, cannot remove
  });

  // Altercase mode: click a case that IS already set → exit altercase, never removes
  it('altercase mode: click case already set → exit altercase, keep all case bits (never removes)', () => {
    // bits=1|4|64; click lowercase (already in case bits) → exit alt, keep 1|4
    expect(toggleCase(1 | 4 | 64, 1)).toBe(1 | 4);
    // bits=1|4|64; click uppercase (already in case bits) → exit alt, keep 1|4
    expect(toggleCase(1 | 4 | 64, 4)).toBe(1 | 4);
  });

  // Altercase mode: click a case that is NOT set → exit altercase and ensure that case is added
  it('altercase mode: click case not in case bits → exit altercase + add case bit', () => {
    // bits=1|4|64; click titlecase (not in case bits) → exit alt + add 2
    expect(toggleCase(1 | 4 | 64, 2)).toBe(1 | 4 | 2);
    // bits=1|4|128; click titlecase → exit alt + add 2
    expect(toggleCase(1 | 4 | 128, 2)).toBe(1 | 4 | 2);
  });

  // Altercase mode: click case when only altercase bit set (no case bits preserved)
  it('altercase mode: no preserved cases → exit altercase + ensure caseBit is set', () => {
    expect(toggleCase(64, 1)).toBe(1);   // exit + ensure lowercase
    expect(toggleCase(64, 4)).toBe(4);   // exit + ensure uppercase
    expect(toggleCase(128, 2)).toBe(2);  // exit + ensure titlecase
  });
});

// ── toggleAltercase ───────────────────────────────────────────────────────────

describe('toggleAltercase', () => {
  // Enter altercase from case mode: preserves case bits
  it('entering altercase from case mode preserves existing case bits', () => {
    expect(toggleAltercase(1 | 4, 64)).toBe(1 | 4 | 64);
    expect(toggleAltercase(1 | 4, 128)).toBe(1 | 4 | 128);
    expect(toggleAltercase(1 | 2 | 4, 64)).toBe(1 | 2 | 4 | 64);
  });

  // Click active radio → exit altercase, restore case bits
  it('clicking active altercase radio exits → bits & ~ALTERCASE_MASK', () => {
    expect(toggleAltercase(1 | 4 | 64, 64)).toBe(1 | 4);
    expect(toggleAltercase(1 | 4 | 128, 128)).toBe(1 | 4);
    expect(toggleAltercase(2 | 64, 64)).toBe(2);
  });

  // Switch between altercase radios
  it('switching altercase radio preserves case bits', () => {
    // from 64 to 128
    expect(toggleAltercase(1 | 4 | 64, 128)).toBe(1 | 4 | 128);
    // from 128 to 64
    expect(toggleAltercase(1 | 4 | 128, 64)).toBe(1 | 4 | 64);
  });

  // Exit with zero preserved cases → fall back to 1 (lowercase)
  it('exiting altercase with no preserved cases falls back to 1 (lowercase)', () => {
    expect(toggleAltercase(64, 64)).toBe(1);    // bits=64, exit → 0 → fallback to 1
    expect(toggleAltercase(128, 128)).toBe(1);  // bits=128, exit → 0 → fallback to 1
  });

  // Enter altercase when only altercase bit was set (no case bits)
  it('entering different altercase when no case bits are set', () => {
    // bits=64 (no cases), enter 128 → 128 (no case bits preserved)
    expect(toggleAltercase(64, 128)).toBe(128);
    // bits=128 (no cases), enter 64 → 64
    expect(toggleAltercase(128, 64)).toBe(64);
  });
});
