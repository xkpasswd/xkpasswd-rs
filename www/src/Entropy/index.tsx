import { pluralize } from 'src/utils';
import './styles.css';

import type * as xktypes from 'src/types/xkpasswd';

// ── thresholds (unchanged from original) ──────────────────────────────────

const NOT_BAD_ENTROPY_BLIND = 78;
const NOT_BAD_ENTROPY_SEEN = 52;
const GREAT_ENTROPY_BLIND = NOT_BAD_ENTROPY_BLIND * 1.5;
const GREAT_ENTROPY_SEEN = NOT_BAD_ENTROPY_SEEN * 1.5;

// ── types ─────────────────────────────────────────────────────────────────

type Props = {
  entropy?: xktypes.Entropy;
};

type FaceKind = 'great' | 'ok' | 'bad';

// ── stick-figure face SVG ──────────────────────────────────────────────────

const FACE_COLORS: Record<FaceKind, string> = {
  great: '#5af78e', // green
  ok: '#9aedfe', // cyan
  bad: '#ff5c57', // red
};

const FACE_MOUTHS: Record<FaceKind, string> = {
  great: 'M8 14 Q12 18 16 14', // smile
  ok: 'M8 15 H16', // flat
  bad: 'M8 16 Q12 12 16 16', // frown
};

/**
 * Inline stick-figure face keyed by verdict.
 * aria-hidden — the verdict text already conveys the meaning.
 */
const StickFace = ({ kind }: { kind: FaceKind }) => {
  const color = FACE_COLORS[kind];
  return (
    <svg
      viewBox="0 0 24 24"
      fill="none"
      stroke={color}
      strokeWidth="1.6"
      strokeLinecap="round"
      aria-hidden="true"
      style={{
        display: 'inline-block',
        width: '18px',
        height: '18px',
        verticalAlign: '-4px',
        marginLeft: '3px',
      }}
    >
      {/* head */}
      <circle cx="12" cy="11" r="9" />
      {/* eyes */}
      <circle cx="9" cy="9" r=".6" fill={color} />
      <circle cx="15" cy="9" r=".6" fill={color} />
      {/* mouth */}
      <path d={FACE_MOUTHS[kind]} />
    </svg>
  );
};

// ── chip (coloured entropy badge) ─────────────────────────────────────────

type ChipProps = {
  value: number;
  kind: 'good' | 'bad';
};

const Chip = ({ value, kind }: ChipProps) => (
  <span className={`chip chip-${kind}`}>{value}</span>
);

// ── guess-time text builder ────────────────────────────────────────────────

/**
 * Converts the WASM GuessTime struct into a human-readable string.
 * Preserves the original logic (calcExceptionalTime + year/month/day comps).
 */
function buildGuessTimeText(value: xktypes.GuessTime): string {
  if (value.years > 1_000_000_000) return 'more than a billion years';
  if (value.years > 1_000_000) return 'more than a million years';
  if (value.years > 1_000) return 'more than a thousand years';

  const parts: string[] = [];
  if (value.years > 0)
    parts.push(`${value.years} ${pluralize(value.years, 'year')}`);
  if (value.months > 0)
    parts.push(`${value.months} ${pluralize(value.months, 'month')}`);
  if (value.days > 0)
    parts.push(`${value.days} ${pluralize(value.days, 'day')}`);

  return parts.length > 0 ? parts.join(' ') : 'less than a day';
}

// ── main component ────────────────────────────────────────────────────────

/**
 * Entropy section — rendered as a `#`-commented stderr block.
 *
 * Thresholds: NOT_BAD_BLIND=78, NOT_BAD_SEEN=52, GREAT=×1.5.
 * Data comes from the WASM Entropy struct (blind_min, blind_max, seen, guess_time).
 */
const Entropy = ({ entropy }: Props) => {
  if (!entropy) return null;

  // ── verdict ──────────────────────────────────────────────────────────────
  const isGreat =
    entropy.blind_min >= GREAT_ENTROPY_BLIND &&
    entropy.seen >= GREAT_ENTROPY_SEEN;
  const isNotBad =
    entropy.blind_min >= NOT_BAD_ENTROPY_BLIND &&
    entropy.seen >= NOT_BAD_ENTROPY_SEEN;

  const verdictText = isGreat ? 'Great!' : isNotBad ? 'Not bad!' : 'Not good!';
  const faceKind: FaceKind = isGreat ? 'great' : isNotBad ? 'ok' : 'bad';

  // ── chip good/bad for each field ─────────────────────────────────────────
  const blindMinGood = entropy.blind_min >= NOT_BAD_ENTROPY_BLIND;
  const blindMaxGood = entropy.blind_max >= NOT_BAD_ENTROPY_BLIND;
  const seenGood = entropy.seen >= NOT_BAD_ENTROPY_SEEN;

  const guessTimeText = buildGuessTimeText(entropy.guess_time);

  return (
    <div className="stderr">
      {/* ── Line 1: entropy values + guess time + verdict ── */}
      <p className="c">
        <span className="h"># </span>
        {'Btw, its entropy is '}
        {entropy.blind_min === entropy.blind_max ? (
          <>
            {'of '}
            <Chip
              value={entropy.blind_min}
              kind={blindMinGood ? 'good' : 'bad'}
            />
          </>
        ) : (
          <>
            {'between '}
            <Chip
              value={entropy.blind_min}
              kind={blindMinGood ? 'good' : 'bad'}
            />
            {' and '}
            <Chip
              value={entropy.blind_max}
              kind={blindMaxGood ? 'good' : 'bad'}
            />
          </>
        )}
        {' bits blind & '}
        <Chip value={entropy.seen} kind={seenGood ? 'good' : 'bad'} />
        {' bits with full knowledge, which takes a computer '}
        <b>{guessTimeText}</b>
        {' to break at 1000 guesses/sec. '}
        {verdictText}
        <StickFace kind={faceKind} />
      </p>

      {/* ── Line 2: recommendation ── */}
      <p className="c gap">
        <span className="h"># </span>
        {"It's recommended to keep "}
        <Chip value={NOT_BAD_ENTROPY_BLIND} kind="good" />
        {' bits & '}
        <Chip value={NOT_BAD_ENTROPY_SEEN} kind="good" />
        {' bits, respectively.'}
      </p>
    </div>
  );
};

export default Entropy;
