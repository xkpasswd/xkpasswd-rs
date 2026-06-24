/**
 * Caret-safe inline `<input>` components for the command-line ControlPanel.
 *
 * ## Why uncontrolled?
 * Controlled inputs (`value=`) lose the caret position on every parent
 * re-render — mid-keystroke re-renders move the cursor to the end.  These
 * components use `defaultValue` + `ref` (uncontrolled pattern) so the DOM
 * owns the text, and all updates (width, value) are applied via direct DOM
 * mutations rather than Preact state changes.
 *
 * The parent (Phase 4c) will additionally `memo` the token list so that live
 * `onLiveChange` callbacks don't trigger a re-render of the token components
 * at all during typing.
 */
import { useCallback, useEffect, useRef } from 'preact/hooks';
import { clampNumber, normalizeNumber, normalizePool } from './editing';

// ── Width helper ──────────────────────────────────────────────────────────────

/** Convert a character count to a `ch`-unit CSS width string.
 *  Always at least 1ch wide. */
function chWidth(len: number): string {
  return `${Math.max(len, 1)}ch`;
}

// ── Shared Tailwind classes ───────────────────────────────────────────────────

/** Base classes applied to every editable input. */
const BASE_CLASSES =
  'inline-block bg-transparent font-mono border-b border-dotted border-current focus:border-solid caret-current outline-none';

// ── EditableNumber ────────────────────────────────────────────────────────────

type EditableNumberProps = {
  /** Committed (external) value — updated by the parent on blur. */
  value: number;
  min: number;
  max: number;
  /** Called on blur with the normalised, committed value. */
  onChange: (v: number) => void;
  /** Called on each keystroke with the live-clamped value (optional). */
  onLiveChange?: (v: number) => void;
  className?: string;
  ariaLabel?: string;
  /**
   * Value to use when the field is blurred empty.
   * Defaults to `min` if not provided.
   */
  emptyDefault?: number;
};

/**
 * Caret-safe inline numeric editor.
 *
 * - Renders as uncontrolled `<input type="text" inputMode="numeric">`.
 * - Width tracks content length via direct DOM mutation (no re-render).
 * - Normalises on blur; reverts on Escape.
 */
export const EditableNumber = ({
  value,
  min,
  max,
  onChange,
  onLiveChange,
  className,
  ariaLabel,
  emptyDefault,
}: EditableNumberProps) => {
  const inputRef = useRef<HTMLInputElement>(null);
  /** Tracks the last value that was committed to the parent. */
  const committedValueRef = useRef(value);

  // Sync external value changes (e.g. preset switch) into the uncontrolled input.
  useEffect(() => {
    const el = inputRef.current;
    if (!el) return;
    if (value !== committedValueRef.current) {
      committedValueRef.current = value;
      el.value = String(value);
      el.style.width = chWidth(String(value).length);
    }
  }, [value]);

  const handleInput = useCallback(
    (e: Event) => {
      const el = e.target as HTMLInputElement;
      const clamped = clampNumber(el.value, min, max);
      // Width follows the raw input text length so the box doesn't jump while typing.
      el.style.width = chWidth(el.value.length);
      onLiveChange?.(clamped);
    },
    [min, max, onLiveChange]
  );

  const handleBlur = useCallback(
    (e: Event) => {
      const el = e.target as HTMLInputElement;
      const fallback = emptyDefault !== undefined ? emptyDefault : min;
      const normalised = normalizeNumber(el.value, min, max, fallback);
      committedValueRef.current = normalised;
      el.value = String(normalised);
      el.style.width = chWidth(String(normalised).length);
      onChange(normalised);
    },
    [min, max, emptyDefault, onChange]
  );

  const handleKeyDown = useCallback((e: Event) => {
    const ke = e as KeyboardEvent;
    const el = e.target as HTMLInputElement;
    if (ke.key === 'Enter') {
      el.blur();
    } else if (ke.key === 'Escape') {
      el.value = String(committedValueRef.current);
      el.style.width = chWidth(String(committedValueRef.current).length);
      el.blur();
    }
  }, []);

  const initialWidth = chWidth(String(value).length);

  return (
    <input
      type="text"
      inputMode="numeric"
      defaultValue={String(value)}
      ref={inputRef}
      className={[BASE_CLASSES, className].filter(Boolean).join(' ')}
      style={{ width: initialWidth }}
      aria-label={ariaLabel}
      onInput={handleInput}
      onBlur={handleBlur}
      onKeyDown={handleKeyDown}
    />
  );
};

// ── EditableString ────────────────────────────────────────────────────────────

type EditableStringProps = {
  /** Committed (external) value. */
  value: string;
  /** Fallback used when the field is blurred empty (e.g. '.' or '%'). */
  fallback: string;
  /** Called on blur with the normalised, committed value. */
  onChange: (v: string) => void;
  /** Called on each keystroke with the trimmed value, when non-empty (optional). */
  onLiveChange?: (v: string) => void;
  className?: string;
  /**
   * When true, wraps the `<input>` in a `<span className="str-quote">` whose
   * `::before`/`::after` pseudo-elements render the surrounding quote glyphs.
   * Use this instead of literal `<span>&quot;</span>` siblings — it keeps the
   * icon footprint identical in both quoted and unquoted states and avoids
   * layout shift caused by the different widths.
   */
  quoted?: boolean;
};

/**
 * Caret-safe inline string/pool editor.
 *
 * Same uncontrolled pattern as EditableNumber but for string fields
 * (separators, symbol pools).  Trims whitespace; falls back on blur if empty.
 */
export const EditableString = ({
  value,
  fallback,
  onChange,
  onLiveChange,
  className,
  quoted,
}: EditableStringProps) => {
  const inputRef = useRef<HTMLInputElement>(null);
  const committedValueRef = useRef(value);

  // Sync external value changes into the uncontrolled input.
  useEffect(() => {
    const el = inputRef.current;
    if (!el) return;
    if (value !== committedValueRef.current) {
      committedValueRef.current = value;
      el.value = value;
      el.style.width = chWidth(value.length);
    }
  }, [value]);

  const handleInput = useCallback(
    (e: Event) => {
      const el = e.target as HTMLInputElement;
      el.style.width = chWidth(el.value.length);
      const trimmed = el.value.trim();
      if (trimmed.length > 0) {
        onLiveChange?.(trimmed);
      }
    },
    [onLiveChange]
  );

  const handleBlur = useCallback(
    (e: Event) => {
      const el = e.target as HTMLInputElement;
      const normalised = normalizePool(el.value, fallback);
      committedValueRef.current = normalised;
      el.value = normalised;
      el.style.width = chWidth(normalised.length);
      onChange(normalised);
    },
    [fallback, onChange]
  );

  const handleKeyDown = useCallback((e: Event) => {
    const ke = e as KeyboardEvent;
    const el = e.target as HTMLInputElement;
    if (ke.key === 'Enter') {
      el.blur();
    } else if (ke.key === 'Escape') {
      el.value = committedValueRef.current;
      el.style.width = chWidth(committedValueRef.current.length);
      el.blur();
    }
  }, []);

  const initialWidth = chWidth(value.length);

  const input = (
    <input
      type="text"
      defaultValue={value}
      ref={inputRef}
      className={[BASE_CLASSES, className].filter(Boolean).join(' ')}
      style={{ width: initialWidth }}
      onInput={handleInput}
      onBlur={handleBlur}
      onKeyDown={handleKeyDown}
    />
  );

  if (quoted) {
    return <span className="str-quote">{input}</span>;
  }

  return input;
};
