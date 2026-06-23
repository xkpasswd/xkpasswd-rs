/**
 * Pure field/transform helpers for the command-line ControlPanel.
 *
 * INTENTIONALLY free of Preact and src/wasm imports: this file runs in the
 * node-based vitest suite and in the browser. It mirrors the numeric values of
 * the Rust WordTransform enum (the wasm module is the canonical source of truth
 * but requires a browser environment to load).
 *
 * WordTransform numeric values (bit-flags):
 *   Lowercase=1, Titlecase=2, Uppercase=4, InversedTitlecase=8
 *   AltercaseLowerFirst=64, AltercaseUpperFirst=128
 */

// ── Field constraints ─────────────────────────────────────────────────────────

/** Per-field numeric constraints [min, max] and empty-blur defaults. */
export const FIELD: Readonly<
  Record<
    | 'wordsCount'
    | 'digitsBefore'
    | 'digitsAfter'
    | 'symbolsBefore'
    | 'symbolsAfter'
    | 'adaptiveCount',
    { min: number; max: number; default: number }
  >
> = {
  wordsCount: { min: 1, max: 10, default: 3 },
  digitsBefore: { min: 0, max: 10, default: 0 },
  digitsAfter: { min: 0, max: 10, default: 2 },
  symbolsBefore: { min: 0, max: 10, default: 0 },
  symbolsAfter: { min: 0, max: 10, default: 2 },
  adaptiveCount: { min: 4, max: 64, default: 32 }, // engine errors on 0
};

// ── Numeric field helpers ─────────────────────────────────────────────────────

/**
 * Strip non-digit characters, parse, and clamp to [min, max].
 * Empty input (no digits) returns min — safe for live-typing where the field
 * may temporarily be empty mid-keystroke.
 */
export function clampNumber(raw: string, min: number, max: number): number {
  const digits = raw.replace(/\D/g, '');
  if (digits === '') return min;
  return Math.min(Math.max(parseInt(digits, 10), min), max);
}

/**
 * On blur: empty string → fallbackDefault; otherwise delegates to clampNumber.
 * The caller supplies the fallbackDefault (typically the field's `.default`
 * from FIELD, or `min` if no default is meaningful).
 */
export function normalizeNumber(
  raw: string,
  min: number,
  max: number,
  fallbackDefault: number
): number {
  const digits = raw.replace(/\D/g, '');
  if (digits === '') return fallbackDefault;
  return clampNumber(raw, min, max);
}

/**
 * Pool/separator string normalization: trim whitespace, fall back to the
 * provided fallback when the result is empty. No deduplication is applied.
 */
export function normalizePool(raw: string, fallback: string): string {
  const trimmed = raw.trim();
  return trimmed === '' ? fallback : trimmed;
}

// ── Word-transform bitflag helpers ────────────────────────────────────────────

// Canonical SINGLE transforms in order (lowercase, titlecase, uppercase, inversed-titlecase).
const SINGLE_TRANSFORMS = [1, 2, 4, 8] as const;
// ALTERCASE transforms (mutually exclusive with singles in practice).
const ALTERCASE_TRANSFORMS = [64, 128] as const;
// Canonical iteration order for activeTransforms output.
const ALL_TRANSFORMS = [
  ...SINGLE_TRANSFORMS,
  ...ALTERCASE_TRANSFORMS,
] as const;

/**
 * Returns the active transform flags from `bits` in canonical order
 * [1, 2, 4, 8, 64, 128].  No duplicates are possible (it's a bitflag set).
 */
export function activeTransforms(bits: number): number[] {
  return ALL_TRANSFORMS.filter((t) => (bits & t) !== 0);
}

/**
 * Replace the active transform at `index` (into `activeTransforms(bits)`) with
 * the "next" unused transform of the same kind:
 *
 * - ALTERCASE flag → toggle to the OTHER altercase flag; result is that flag only.
 * - SINGLE → advance to the next SINGLE not currently in the set (wrapping around
 *   the 4 singles). No-op when all 4 singles are already active.
 */
export function cycleTransform(bits: number, index: number): number {
  const active = activeTransforms(bits);
  const current = active[index];

  // ALTERCASE toggle: result is just the other altercase flag.
  if (current === 64) return 128;
  if (current === 128) return 64;

  // SINGLE: find next single (after current, wrapping) not in bits.
  const pos = SINGLE_TRANSFORMS.indexOf(current as 1 | 2 | 4 | 8);
  for (let i = 1; i <= SINGLE_TRANSFORMS.length; i++) {
    const next = SINGLE_TRANSFORMS[(pos + i) % SINGLE_TRANSFORMS.length];
    if ((bits & next) === 0) {
      return (bits & ~current) | next;
    }
  }

  // All 4 singles are already set — no-op.
  return bits;
}

/**
 * Add the first unused SINGLE transform (in order 1, 2, 4, 8).
 * No-op when:
 *   - any ALTERCASE flag is active (altercase and singles are exclusive modes), or
 *   - all 4 singles are already present.
 */
export function addTransform(bits: number): number {
  // No-op in altercase mode.
  if (ALTERCASE_TRANSFORMS.some((t) => (bits & t) !== 0)) return bits;

  const unused = SINGLE_TRANSFORMS.find((t) => (bits & t) === 0);
  if (unused === undefined) return bits; // all 4 singles present

  return bits | unused;
}

/**
 * Clear the active flag at `index` (into `activeTransforms(bits)`).
 * Enforces a minimum of 1 active transform — no-op when only one is active.
 */
export function removeTransform(bits: number, index: number): number {
  const active = activeTransforms(bits);
  if (active.length <= 1) return bits; // enforce min 1

  const current = active[index];
  return bits & ~current;
}
