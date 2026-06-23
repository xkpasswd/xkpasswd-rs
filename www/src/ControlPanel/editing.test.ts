import { describe, it, expect } from 'vitest';
import {
  FIELD,
  clampNumber,
  normalizeNumber,
  normalizePool,
  activeTransforms,
  cycleTransform,
  addTransform,
  removeTransform,
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

// ── activeTransforms ─────────────────────────────────────────────────────────

describe('activeTransforms', () => {
  it('0 → empty array', () => {
    expect(activeTransforms(0)).toEqual([]);
  });

  it('single SINGLE flag → [flag]', () => {
    expect(activeTransforms(1)).toEqual([1]);   // lowercase
    expect(activeTransforms(2)).toEqual([2]);   // titlecase
    expect(activeTransforms(4)).toEqual([4]);   // uppercase
    expect(activeTransforms(8)).toEqual([8]);   // inversed-titlecase
  });

  it('single ALTERCASE flag → [flag]', () => {
    expect(activeTransforms(64)).toEqual([64]);   // altercase-lower-first
    expect(activeTransforms(128)).toEqual([128]); // altercase-upper-first
  });

  it('multiple singles in canonical order [1,2,4,8]', () => {
    expect(activeTransforms(1 | 2)).toEqual([1, 2]);
    expect(activeTransforms(1 | 4)).toEqual([1, 4]);
    expect(activeTransforms(2 | 8)).toEqual([2, 8]);
    expect(activeTransforms(1 | 2 | 4 | 8)).toEqual([1, 2, 4, 8]);
  });

  it('bits in any order → canonical order always', () => {
    // 8 | 1 = 9, should still return [1, 8]
    expect(activeTransforms(8 | 1)).toEqual([1, 8]);
    expect(activeTransforms(4 | 2 | 1)).toEqual([1, 2, 4]);
  });
});

// ── cycleTransform ────────────────────────────────────────────────────────────

describe('cycleTransform', () => {
  // ALTERCASE toggle
  it('ALTERCASE-lower (64) at index 0 → toggle to ALTERCASE-upper (128 only)', () => {
    expect(cycleTransform(64, 0)).toBe(128);
  });

  it('ALTERCASE-upper (128) at index 0 → toggle to ALTERCASE-lower (64 only)', () => {
    expect(cycleTransform(128, 0)).toBe(64);
  });

  // SINGLE cycling
  it('single lowercase (1): cycle index 0 → next unused single = 2 (titlecase)', () => {
    expect(cycleTransform(1, 0)).toBe(2);
  });

  it('single titlecase (2): cycle index 0 → next unused = 4 (uppercase)', () => {
    expect(cycleTransform(2, 0)).toBe(4);
  });

  it('single uppercase (4): cycle index 0 → next unused = 8 (inversed-titlecase)', () => {
    expect(cycleTransform(4, 0)).toBe(8);
  });

  it('single inversed-titlecase (8): cycle index 0 → wraps to 1 (lowercase)', () => {
    expect(cycleTransform(8, 0)).toBe(1);
  });

  it('lowercase+titlecase (1|2): cycle index 0 (bit 1) → replace 1 with 4 (next unused)', () => {
    // active = [1,2], cycle index 0 → find next after 1 not in {2}: 4
    expect(cycleTransform(1 | 2, 0)).toBe((2 | 4));
  });

  it('lowercase+titlecase (1|2): cycle index 1 (bit 2) → replace 2 with 4 (next unused)', () => {
    // active = [1,2], cycle index 1 → find next after 2 not in {1}: 4
    expect(cycleTransform(1 | 2, 1)).toBe((1 | 4));
  });

  it('lowercase+uppercase (1|4): cycle index 1 (bit 4) → next after 4 not in {1}: 8', () => {
    // active = [1,4], cycle index 1 → find next after 4 not in {1}: 8
    expect(cycleTransform(1 | 4, 1)).toBe((1 | 8));
  });

  it('wrap: uppercase+inversed (4|8): cycle index 1 (bit 8) → wraps to 1 (not in {4})', () => {
    // active = [4,8], cycle index 1 → find next after 8 wrapping: 1 not in {4}
    expect(cycleTransform(4 | 8, 1)).toBe((4 | 1));
  });

  it('no-op when all 4 singles set: cycle returns unchanged bits', () => {
    const all4 = 1 | 2 | 4 | 8; // 15
    expect(cycleTransform(all4, 0)).toBe(all4);
    expect(cycleTransform(all4, 1)).toBe(all4);
    expect(cycleTransform(all4, 2)).toBe(all4);
    expect(cycleTransform(all4, 3)).toBe(all4);
  });

  it('cycle index 2 in a 3-single set: replaces that specific flag', () => {
    // bits=1|2|4, active=[1,2,4], cycle index 2 (bit 4) → next after 4 not in {1,2}: 8
    expect(cycleTransform(1 | 2 | 4, 2)).toBe((1 | 2 | 8));
  });
});

// ── addTransform ──────────────────────────────────────────────────────────────

describe('addTransform', () => {
  it('bits=0 → add first SINGLE (1=lowercase)', () => {
    expect(addTransform(0)).toBe(1);
  });

  it('bits=1 (lowercase) → add next unused: 2 (titlecase)', () => {
    expect(addTransform(1)).toBe(1 | 2);
  });

  it('bits=1|2 → add next unused: 4 (uppercase)', () => {
    expect(addTransform(1 | 2)).toBe(1 | 2 | 4);
  });

  it('bits=1|2|4 → add next unused: 8 (inversed-titlecase)', () => {
    expect(addTransform(1 | 2 | 4)).toBe(1 | 2 | 4 | 8);
  });

  it('no-op when all 4 singles present (1|2|4|8)', () => {
    const all4 = 1 | 2 | 4 | 8;
    expect(addTransform(all4)).toBe(all4);
  });

  it('no-op in altercase mode (bit 64)', () => {
    expect(addTransform(64)).toBe(64);
  });

  it('no-op in altercase mode (bit 128)', () => {
    expect(addTransform(128)).toBe(128);
  });

  it('adds first gap in a non-contiguous set', () => {
    // bits=1|4 → first unused is 2 (titlecase)
    expect(addTransform(1 | 4)).toBe(1 | 2 | 4);
  });
});

// ── removeTransform ──────────────────────────────────────────────────────────

describe('removeTransform', () => {
  it('no-op if only one active (single): stays unchanged', () => {
    expect(removeTransform(1, 0)).toBe(1);
    expect(removeTransform(2, 0)).toBe(2);
    expect(removeTransform(8, 0)).toBe(8);
  });

  it('no-op if only one active (altercase): stays unchanged', () => {
    expect(removeTransform(64, 0)).toBe(64);
    expect(removeTransform(128, 0)).toBe(128);
  });

  it('two singles: remove index 0 (first)', () => {
    // active=[1,2], remove index 0 → remove bit 1 → result=2
    expect(removeTransform(1 | 2, 0)).toBe(2);
  });

  it('two singles: remove index 1 (second)', () => {
    // active=[1,2], remove index 1 → remove bit 2 → result=1
    expect(removeTransform(1 | 2, 1)).toBe(1);
  });

  it('three singles: remove middle (index 1)', () => {
    // active=[1,2,4], remove index 1 → remove bit 2 → result=1|4=5
    expect(removeTransform(1 | 2 | 4, 1)).toBe(1 | 4);
  });

  it('three singles: remove last (index 2)', () => {
    // active=[1,2,4], remove index 2 → remove bit 4 → result=1|2=3
    expect(removeTransform(1 | 2 | 4, 2)).toBe(1 | 2);
  });

  it('four singles: remove any → 3 remain', () => {
    const all4 = 1 | 2 | 4 | 8;
    expect(removeTransform(all4, 0)).toBe(2 | 4 | 8);
    expect(removeTransform(all4, 3)).toBe(1 | 2 | 4);
  });
});
