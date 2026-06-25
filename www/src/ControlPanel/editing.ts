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

/** Case transforms in display order: lowercase, Titlecase, UPPERCASE, iNVERSED TITLECASE. */
export const CASE_TRANSFORMS = [1, 2, 4, 8] as const;

/** Altercase transforms (deterministic word-level alternation; mutually exclusive with cases). */
export const ALTERCASE_LOWER = 64;
export const ALTERCASE_UPPER = 128;

/** Mask covering both altercase bits. */
const ALTERCASE_MASK = ALTERCASE_LOWER | ALTERCASE_UPPER;

/**
 * Returns the canonical active altercase bit: `64` wins if both bits are set
 * (mirrors the Rust engine's `with_word_transforms` early-return logic).
 *
 * Returns `0` when no altercase bit is active.
 */
export function activeAltercase(bits: number): number {
  if (bits & ALTERCASE_LOWER) return ALTERCASE_LOWER;
  if (bits & ALTERCASE_UPPER) return ALTERCASE_UPPER;
  return 0;
}

/**
 * Canonical projection of `wordTransforms` bits → ordered array of active flags.
 *
 * - Altercase mode (any altercase bit set): `[altBit]` only — case bits are
 *   suppressed because the engine ignores them when an altercase bit is present.
 * - Case mode: `CASE_TRANSFORMS` entries that are set, in display order.
 *
 * Used by the command-string builder and token display.
 */
export function canonicalTransforms(bits: number): number[] {
  const alt = activeAltercase(bits);
  if (alt !== 0) return [alt];
  return CASE_TRANSFORMS.filter((b) => (bits & b) !== 0);
}

/**
 * Returns the currently-selected case bits in canonical display order.
 * Altercase bits are always ignored — the result is the preserved case
 * selection regardless of whether altercase mode is currently active.
 */
export function selectedCases(bits: number): number[] {
  return CASE_TRANSFORMS.filter((b) => (bits & b) !== 0);
}

/**
 * Popover handler: toggle one case checkbox. Returns new `wordTransforms` bits.
 *
 * - **Altercase active:** exit altercase AND ensure `caseBit` is set.
 *   `(bits & ~ALTERCASE_MASK) | caseBit` — NEVER removes a case bit.
 * - **Case mode, `caseBit` already set:** remove it, unless it's the sole
 *   selected case (lock-last rule enforces ≥1 case).
 * - **Case mode, `caseBit` not set:** add it.
 */
export function toggleCase(bits: number, caseBit: number): number {
  if (activeAltercase(bits)) {
    // Exit altercase and ensure this case bit is set (never removes).
    return (bits & ~ALTERCASE_MASK) | caseBit;
  }
  if (bits & caseBit) {
    // Lock-last: no-op when this would remove the only remaining case.
    if (selectedCases(bits).length <= 1) return bits;
    return bits & ~caseBit;
  }
  return bits | caseBit;
}

/**
 * Popover handler: click one altercase radio. Returns new `wordTransforms` bits.
 *
 * - **That altercase bit already active:** exit → `bits & ~ALTERCASE_MASK`.
 *   If that yields `0` (no case bits were preserved), fall back to `1` (lowercase).
 * - **Other (inactive) radio clicked:** set this altercase bit, clear the other,
 *   and PRESERVE existing case bits → `(bits & ~ALTERCASE_MASK) | altBit`.
 */
export function toggleAltercase(bits: number, altBit: number): number {
  if (activeAltercase(bits) === altBit) {
    const caseOnly = bits & ~ALTERCASE_MASK;
    return caseOnly === 0 ? 1 : caseOnly;
  }
  return (bits & ~ALTERCASE_MASK) | altBit;
}
