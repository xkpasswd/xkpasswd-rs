/**
 * Password colorizer — pure, never throws.
 *
 * Core invariant: segments.map(s => s.text).join('') === passwd
 */

export type SegKind = 'word' | 'sep' | 'digit' | 'symbol';
export interface Segment {
  text: string;
  kind: SegKind;
}

export interface SegmentInputs {
  wordsCount: number;
  separators: string;
  digitsBefore: number;
  digitsAfter: number;
  /** Already 0 in adaptive mode. */
  symbolsBefore: number;
  /** Already 0 in adaptive mode. */
  symbolsAfter: number;
  paddingSymbols: string;
  adaptivePadding: boolean;
  adaptiveCount: number;
}

/**
 * Minimal builder shape — structurally compatible with SettingsBuilderType
 * from contexts.tsx (TypeScript structural typing ensures assignability).
 */
type BuilderShape = Readonly<{
  preset?: number;
  wordsCount: number;
  wordTransforms: number;
  separators: string;
  digitsBefore: number;
  digitsAfter: number;
  symbolsBefore: number;
  symbolsAfter: number;
  paddingSymbols: string;
  adaptivePadding: boolean;
  adaptiveCount: number;
}>;

// ── regex constants ────────────────────────────────────────────────────────

const RE_ALPHA = /^[a-zA-Z]+$/;
const RE_DIGIT = /^[0-9]+$/;
const RE_ALNUM = /^[a-zA-Z0-9]+$/;
const RE_ALPHA_CHAR = /[a-zA-Z]/;
const RE_DIGIT_CHAR = /[0-9]/;

// ── internal helpers ───────────────────────────────────────────────────────

/**
 * Returns true when every character in `text` is the same char AND that char
 * is present in `pool`.  Empty text is considered homogeneous.
 */
function isHomogeneous(text: string, pool: string): boolean {
  if (text.length === 0) return true;
  const ch = text[0];
  if (!pool.includes(ch)) return false;
  for (let i = 1; i < text.length; i++) {
    if (text[i] !== ch) return false;
  }
  return true;
}

/**
 * Char-class fallback.  Scans left-to-right, coalesces runs by kind.
 * Classification (in priority order):
 *   [a-zA-Z]                   → word
 *   [0-9]                      → digit
 *   in separators AND symbols  → sep
 *   in separators only         → sep
 *   in symbols only            → symbol
 *   anything else              → word   (guarantees exact concat)
 */
function charClassFallback(
  text: string,
  separators: string,
  symbols: string
): Segment[] {
  if (text.length === 0) return [];

  const segments: Segment[] = [];
  let current = '';
  let currentKind: SegKind = 'word';

  for (let i = 0; i < text.length; i++) {
    const ch = text[i];
    let kind: SegKind;

    if (RE_ALPHA_CHAR.test(ch)) {
      kind = 'word';
    } else if (RE_DIGIT_CHAR.test(ch)) {
      kind = 'digit';
    } else if (separators.includes(ch)) {
      kind = 'sep'; // covers "in both" and "sep only" cases
    } else if (symbols.includes(ch)) {
      kind = 'symbol';
    } else {
      kind = 'word';
    }

    if (i === 0) {
      current = ch;
      currentKind = kind;
    } else if (kind === currentKind) {
      current += ch;
    } else {
      segments.push({ text: current, kind: currentKind });
      current = ch;
      currentKind = kind;
    }
  }

  if (current.length > 0) {
    segments.push({ text: current, kind: currentKind });
  }

  return segments;
}

/**
 * Try to split `middle` using each unique char in `separators` and find one
 * that yields exactly `expectedParts` non-empty alnum-only parts satisfying
 * the digit-length constraints.
 *
 * When `digitsAfter === 0`, the last part is not required to be a digit block.
 */
function detectSeparator(
  middle: string,
  separators: string,
  expectedParts: number,
  digitsBefore: number,
  digitsAfter: number
): { sep: string; parts: string[] } | null {
  if (expectedParts < 1 || separators.length === 0) return null;

  const seen = new Set<string>();

  for (const sep of separators) {
    if (seen.has(sep)) continue;
    seen.add(sep);

    const parts = middle.split(sep);
    if (parts.length !== expectedParts) continue;
    // All parts must be non-empty alnum
    if (!parts.every((p) => RE_ALNUM.test(p))) continue;

    // digitsBefore constraint
    if (digitsBefore > 0) {
      if (!RE_DIGIT.test(parts[0])) continue;
      if (parts[0].length !== digitsBefore) continue;
    }

    // digitsAfter constraint (only when caller requests it)
    if (digitsAfter > 0) {
      const last = parts[parts.length - 1];
      if (!RE_DIGIT.test(last)) continue;
      if (last.length !== digitsAfter) continue;
    }

    // Interior word parts (between the digit blocks) must be alpha
    const firstWordIdx = digitsBefore > 0 ? 1 : 0;
    const lastWordIdx = digitsAfter > 0 ? parts.length - 2 : parts.length - 1;
    let interiorOk = true;
    for (let i = firstWordIdx; i <= lastWordIdx; i++) {
      if (!RE_ALPHA.test(parts[i])) {
        interiorOk = false;
        break;
      }
    }
    if (!interiorOk) continue;

    return { sep, parts };
  }

  return null;
}

/**
 * Convert detected `parts` + `sep` into a Segment array, tagging digit
 * positions based on `digitsBefore` / `digitsAfter`.
 */
function tagParts(
  parts: string[],
  sep: string,
  digitsBefore: number,
  digitsAfter: number
): Segment[] {
  const segments: Segment[] = [];

  for (let i = 0; i < parts.length; i++) {
    if (i > 0) {
      segments.push({ text: sep, kind: 'sep' });
    }

    let kind: SegKind;
    if (i === 0 && digitsBefore > 0) {
      kind = 'digit';
    } else if (i === parts.length - 1 && digitsAfter > 0) {
      kind = 'digit';
    } else {
      kind = 'word';
    }

    segments.push({ text: parts[i], kind });
  }

  return segments;
}

/**
 * Segment a "middle" string (no outer symbol padding) using separator detection
 * or char-class fallback.
 */
function segmentMiddle(
  middle: string,
  separators: string,
  wordsCount: number,
  digitsBefore: number,
  digitsAfter: number,
  paddingSymbols: string
): Segment[] {
  if (middle.length === 0) return [];

  const expectedParts =
    wordsCount + (digitsBefore > 0 ? 1 : 0) + (digitsAfter > 0 ? 1 : 0);

  if (expectedParts === 0) {
    // Nothing expected in the middle — use fallback
    return charClassFallback(middle, separators, paddingSymbols);
  }

  if (expectedParts === 1 || separators.length === 0) {
    // Single-part case: tag as word or digit; fall back for mixed content
    if (digitsBefore > 0 && RE_DIGIT.test(middle)) {
      return [{ text: middle, kind: 'digit' }];
    }
    if (digitsAfter > 0 && RE_DIGIT.test(middle)) {
      return [{ text: middle, kind: 'digit' }];
    }
    if (wordsCount >= 1 && RE_ALPHA.test(middle)) {
      return [{ text: middle, kind: 'word' }];
    }
    return charClassFallback(middle, separators, paddingSymbols);
  }

  // Try separator detection
  const detected = detectSeparator(
    middle,
    separators,
    expectedParts,
    digitsBefore,
    digitsAfter
  );
  if (detected) {
    return tagParts(detected.parts, detected.sep, digitsBefore, digitsAfter);
  }

  return charClassFallback(middle, separators, paddingSymbols);
}

// ── fixed-mode segmentation ────────────────────────────────────────────────

function segmentFixed(passwd: string, s: SegmentInputs): Segment[] {
  // Sanity: password too short to contain both symbol blocks
  if (passwd.length < s.symbolsBefore + s.symbolsAfter) {
    return charClassFallback(passwd, s.separators, s.paddingSymbols);
  }

  const prefix = passwd.slice(0, s.symbolsBefore);
  const suffix =
    s.symbolsAfter > 0 ? passwd.slice(passwd.length - s.symbolsAfter) : '';
  const middle = passwd.slice(
    s.symbolsBefore,
    s.symbolsAfter > 0 ? passwd.length - s.symbolsAfter : passwd.length
  );

  // Validate prefix/suffix are homogeneous runs of a paddingSymbols char
  if (
    !isHomogeneous(prefix, s.paddingSymbols) ||
    !isHomogeneous(suffix, s.paddingSymbols)
  ) {
    return charClassFallback(passwd, s.separators, s.paddingSymbols);
  }

  const segments: Segment[] = [];

  if (prefix.length > 0) {
    segments.push({ text: prefix, kind: 'symbol' });
  }

  segments.push(
    ...segmentMiddle(
      middle,
      s.separators,
      s.wordsCount,
      s.digitsBefore,
      s.digitsAfter,
      s.paddingSymbols
    )
  );

  if (suffix.length > 0) {
    segments.push({ text: suffix, kind: 'symbol' });
  }

  return segments;
}

// ── adaptive-mode segmentation ─────────────────────────────────────────────

function segmentAdaptive(passwd: string, s: SegmentInputs): Segment[] {
  if (passwd.length === 0) return [];

  const lastChar = passwd[passwd.length - 1];
  const isInSymbols = s.paddingSymbols.includes(lastChar);
  const isInSeps = s.separators.includes(lastChar);
  const isAlNum = RE_ALPHA_CHAR.test(lastChar) || RE_DIGIT_CHAR.test(lastChar);

  // Detect trailing pad: last char must be a paddingSymbol, not a separator,
  // and not alphanumeric.
  if (isInSymbols && !isInSeps && !isAlNum) {
    let padStart = passwd.length - 1;
    while (padStart > 0 && passwd[padStart - 1] === lastChar) {
      padStart--;
    }

    const natural = passwd.slice(0, padStart);
    const padRun = passwd.slice(padStart);

    if (natural.length > 0) {
      const naturalSegments = segmentMiddle(
        natural,
        s.separators,
        s.wordsCount,
        s.digitsBefore,
        s.digitsAfter,
        s.paddingSymbols
      );

      // Only attach the pad run when the natural part produced at least one
      // word or digit segment (otherwise fall through).
      const hasWordOrDigit = naturalSegments.some(
        (seg) => seg.kind === 'word' || seg.kind === 'digit'
      );

      if (hasWordOrDigit) {
        return [...naturalSegments, { text: padRun, kind: 'symbol' }];
      }
    }
    // Fall through to "treat as trimmed"
  }

  // Treat as trimmed / unchanged: try separator detection with decreasing
  // part counts.  digitsAfter is only enforced when count === maxParts
  // (it may have been trimmed away at smaller counts).
  const maxParts =
    s.wordsCount +
    (s.digitsBefore > 0 ? 1 : 0) +
    (s.digitsAfter > 0 ? 1 : 0);

  for (let count = maxParts; count >= 1; count--) {
    const checkDigitsAfter = count === maxParts ? s.digitsAfter : 0;
    const detected = detectSeparator(
      passwd,
      s.separators,
      count,
      s.digitsBefore,
      checkDigitsAfter
    );
    if (detected) {
      return tagParts(
        detected.parts,
        detected.sep,
        s.digitsBefore,
        checkDigitsAfter
      );
    }
  }

  return charClassFallback(passwd, s.separators, s.paddingSymbols);
}

// ── public API ─────────────────────────────────────────────────────────────

/**
 * Segment a password string into typed color segments.
 *
 * Invariant: `segments.map(s => s.text).join('') === passwd` for all inputs.
 * Pure — never throws.
 */
export function segmentPassword(passwd: string, s: SegmentInputs): Segment[] {
  if (passwd.length === 0) return [];

  return s.adaptivePadding
    ? segmentAdaptive(passwd, s)
    : segmentFixed(passwd, s);
}

/**
 * Build SegmentInputs from the shared settings builder.
 *
 * When a named preset is active, returns zeroed inputs that cause
 * `segmentPassword` to fall back to the char-class algorithm, which still
 * satisfies the concat invariant.
 */
export function buildSegmentInputs(
  builder: BuilderShape,
  presetOverride?: number
): SegmentInputs {
  const preset = presetOverride ?? builder.preset;

  if (preset != null) {
    return {
      wordsCount: 0,
      separators: '',
      digitsBefore: 0,
      digitsAfter: 0,
      symbolsBefore: 0,
      symbolsAfter: 0,
      paddingSymbols: '',
      adaptivePadding: false,
      adaptiveCount: 0,
    };
  }

  return {
    wordsCount: builder.wordsCount,
    separators: builder.separators,
    digitsBefore: builder.digitsBefore,
    digitsAfter: builder.digitsAfter,
    symbolsBefore: builder.adaptivePadding ? 0 : builder.symbolsBefore,
    symbolsAfter: builder.adaptivePadding ? 0 : builder.symbolsAfter,
    paddingSymbols: builder.paddingSymbols,
    adaptivePadding: builder.adaptivePadding,
    adaptiveCount: builder.adaptiveCount,
  };
}
