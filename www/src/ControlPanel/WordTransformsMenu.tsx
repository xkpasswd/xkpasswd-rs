/**
 * Multi-select popover for the --transforms flag.
 *
 * Provides a self-contained positioned popover (portal + dismiss) that lets
 * the user:
 *   - Toggle individual case-transform checkboxes (lowercase / Titlecase /
 *     UPPERCASE / iNVERSED TITLECASE)
 *   - Select one of the mutually-exclusive altercase transforms
 *     (altercase LOWER first / ALTERCASE upper FIRST)
 *
 * Position is computed ONCE on mount from the anchor's getBoundingClientRect
 * (mirrors DropdownButton's strategy).  The menu stays in place while the
 * user edits — even when the canonical token set changes due to mode switches.
 */
import { createPortal } from 'preact/compat';
import { useEffect, useRef, useState } from 'preact/hooks';
import {
  ALTERCASE_LOWER,
  ALTERCASE_UPPER,
  CASE_TRANSFORMS,
  activeAltercase,
  selectedCases,
  toggleAltercase,
  toggleCase,
} from './editing';

/**
 * Display labels (self-demonstrating, distinct from kebab CLI flag values).
 * Kept co-located so label ↔ bit mapping is easy to audit.
 */
const TRANSFORM_LABEL: Readonly<Record<number, string>> = {
  1: 'lowercase',
  2: 'Titlecase',
  4: 'UPPERCASE',
  8: 'iNVERSED TITLECASE',
  64: 'altercase LOWER first',
  128: 'ALTERCASE upper FIRST',
};

type Props = {
  /** The trigger element that opened this popover; used for positioning and
   *  to exclude it from outside-click detection. */
  anchor: HTMLElement;
  /** Current wordTransforms bitfield (may include preserved case bits + altercase bit). */
  bits: number;
  /** Called with the new bitfield whenever the user toggles a row. */
  onChange: (next: number) => void;
  /** Called when the user dismisses the popover (Escape, outside click, etc.). */
  onClose: () => void;
};

/**
 * Positioned popover for selecting word transforms.
 * Mirrors DropdownButton's portal/position/dismiss logic.
 */
const WordTransformsPopover = ({ anchor, bits, onChange, onClose }: Props) => {
  const menuRef = useRef<HTMLDivElement>(null);

  // Compute position ONCE on mount from the anchor rect (same approach as
  // DropdownButton's computePositionAndAlignment).  Never re-positions so
  // the menu stays put while the user edits.
  const [pos] = useState<{ top: number; left?: number; right?: number }>(
    () => {
      const rect = anchor.getBoundingClientRect();
      const rightAligned = rect.x + rect.width / 2 > window.innerWidth / 2;
      const p: { top: number; left?: number; right?: number } = {
        top: rect.bottom + 8,
      };
      if (rightAligned) {
        p.right = window.innerWidth - rect.right;
      } else {
        p.left = rect.left;
      }
      return p;
    }
  );

  // Keep a stable ref to onClose so the dismiss effect (below) only
  // re-registers when the anchor changes, not on every render.
  const onCloseRef = useRef(onClose);
  onCloseRef.current = onClose;

  // Dismiss on: pointerdown outside (not the anchor, not the menu),
  //             Escape key, scroll (capture phase), window resize.
  useEffect(() => {
    const handlePointerDown = (e: PointerEvent) => {
      const target = e.target as Node;
      if (
        !anchor.contains(target) &&
        !menuRef.current?.contains(target)
      ) {
        onCloseRef.current();
      }
    };

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onCloseRef.current();
    };

    const handleScrollOrResize = () => onCloseRef.current();

    document.addEventListener('pointerdown', handlePointerDown);
    document.addEventListener('keydown', handleKeyDown);
    window.addEventListener('scroll', handleScrollOrResize, { capture: true });
    window.addEventListener('resize', handleScrollOrResize);

    return () => {
      document.removeEventListener('pointerdown', handlePointerDown);
      document.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('scroll', handleScrollOrResize, {
        capture: true,
      });
      window.removeEventListener('resize', handleScrollOrResize);
    };
  }, [anchor]); // only re-run when the anchor element changes

  // Derive display state from the current bitfield.
  const altActive = activeAltercase(bits);
  const cases = selectedCases(bits);

  const menuStyle: Record<string, string | number> = {
    position: 'fixed',
    top: pos.top,
    zIndex: 50,
  };
  if (pos.right !== undefined) {
    menuStyle.right = pos.right;
  } else {
    menuStyle.left = pos.left ?? 0;
  }

  return createPortal(
    <div
      ref={menuRef}
      className="dropdown-menu tf-menu"
      role="menu"
      tabIndex={-1}
      style={menuStyle}
    >
      {/* ── Case checkboxes ─────────────────────────────────────────── */}
      <div className="tf-cmt top">
        # pick one or more - applied at random, per word
      </div>

      {CASE_TRANSFORMS.map((caseBit) => {
        const isChecked = cases.includes(caseBit);
        // Lock the sole remaining case so the user can't deselect all cases.
        const isLocked = !altActive && cases.length === 1 && isChecked;
        // Grey all case rows when an altercase is active (still clickable —
        // clicking one exits altercase mode).
        const isDisabled = altActive !== 0;

        const rowClasses = [
          'tf-opt',
          isDisabled && 'dis',
          isLocked && 'locked',
        ]
          .filter(Boolean)
          .join(' ');

        const boxClasses = [
          'tf-box',
          isChecked && 'on',
          isLocked && 'locked',
        ]
          .filter(Boolean)
          .join(' ');

        return (
          <div
            key={caseBit}
            className={rowClasses}
            role="checkbox"
            aria-checked={isChecked}
            aria-disabled={isLocked ? true : undefined}
            tabIndex={0}
            onClick={() => onChange(toggleCase(bits, caseBit))}
            onKeyDown={(e: KeyboardEvent) => {
              if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                onChange(toggleCase(bits, caseBit));
              }
            }}
          >
            <span className={boxClasses}>
              <span className="tf-chk" />
            </span>
            <span className="tf-lab">{TRANSFORM_LABEL[caseBit]}</span>
          </div>
        );
      })}

      {/* ── Altercase radios ────────────────────────────────────────── */}
      <div className="tf-cmt alt">
        # ...or pick exactly one altercase - mutually exclusive, replaces all of the above
      </div>

      {[ALTERCASE_LOWER, ALTERCASE_UPPER].map((altBit) => {
        const isChecked = altActive === altBit;

        return (
          <div
            key={altBit}
            className="tf-opt"
            role="radio"
            aria-checked={isChecked}
            tabIndex={0}
            onClick={() => onChange(toggleAltercase(bits, altBit))}
            onKeyDown={(e: KeyboardEvent) => {
              if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                onChange(toggleAltercase(bits, altBit));
              }
            }}
          >
            <span className={`tf-radio${isChecked ? ' on' : ''}`}>
              <span className="tf-dot" />
            </span>
            <span className="tf-lab">{TRANSFORM_LABEL[altBit]}</span>
          </div>
        );
      })}
    </div>,
    document.body
  );
};

export { WordTransformsPopover };
