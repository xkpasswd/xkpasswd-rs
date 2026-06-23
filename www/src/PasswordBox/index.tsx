import { useCallback, useEffect, useRef, useState } from 'preact/hooks';
import { CopyIcon } from 'src/Icons';
import { useSettings } from 'src/contexts';
import { copyToClipboard } from 'src/utils';
import { buildSegmentInputs, segmentPassword, type Segment, type SegKind } from './segment';
import './styles.css';

type Props = {
  passwd: string;
};

/** Map each segment kind to its Tailwind colour class. */
const KIND_CLASS: Record<SegKind, string> = {
  word: 'text-white',
  sep: 'text-faint',
  digit: 'text-blue',
  symbol: 'text-yellow',
};

type ColorizedPasswordProps = {
  segments: Segment[];
};

/**
 * Renders a list of password segments, each wrapped in a <span> with its
 * corresponding syntax-colour class.  JSX auto-escapes text — no manual
 * escaping needed.
 */
const ColorizedPassword = ({ segments }: ColorizedPasswordProps) => (
  <>
    {segments.map((seg, i) => (
      <span key={i} className={KIND_CLASS[seg.kind]}>
        {seg.text}
      </span>
    ))}
  </>
);

/**
 * Stdout row — the generated password displayed as a terminal output line.
 *
 * Tap / click / Enter / Space → copy to clipboard.
 * Preserves the full clipboard fallback chain:
 *   navigator.clipboard  →  execCommand('copy')  →  prompt() fallback
 *
 * A transient "✓ copied" indicator replaces the icon for ~1.1 s after a
 * successful copy.
 */
const PasswordBox = ({ passwd }: Props) => {
  const [copied, setCopied] = useState(false);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const { builder } = useSettings();

  // Colorize the password into typed segments using the shared builder state.
  const segments = passwd
    ? segmentPassword(passwd, buildSegmentInputs(builder))
    : [];

  const copyPasswd = useCallback(() => {
    if (copyToClipboard(passwd)) {
      setCopied(true);
      if (timerRef.current) clearTimeout(timerRef.current);
      timerRef.current = setTimeout(() => setCopied(false), 1100);
    } else {
      // Last-resort fallback: prompt the user to copy manually.
      prompt('Password to copy', passwd);
    }
  }, [passwd]);

  // Clean up the timer on unmount to avoid state updates on dead components.
  useEffect(() => {
    return () => {
      if (timerRef.current) clearTimeout(timerRef.current);
    };
  }, []);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        copyPasswd();
      }
    },
    [copyPasswd]
  );

  if (!passwd) return null;

  return (
    <div
      className="stdout-row"
      role="button"
      tabIndex={0}
      aria-label="Copy password"
      onClick={copyPasswd}
      onKeyDown={handleKeyDown}
    >
      <span className="stdout-password">
        <ColorizedPassword segments={segments} />
      </span>
      {/* aria-hidden: the icon/confirmation is purely decorative; the
          aria-label on the row is sufficient for screen readers. */}
      <span className="stdout-icon" aria-hidden="true">
        {copied ? (
          <span className="stdout-copied">✓ copied</span>
        ) : (
          <CopyIcon className="w-5 h-5" />
        )}
      </span>
    </div>
  );
};

export default PasswordBox;
