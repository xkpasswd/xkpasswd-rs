import { ComponentChildren } from 'preact';
import { createPortal } from 'preact/compat';
import { useCallback, useEffect, useRef, useState } from 'preact/hooks';
import './styles.css';

type ChildrenProps = {
  dismiss: () => void;
};

type Props = {
  name: string;
  title: ComponentChildren;
  className?: string;
  buildDropdownClassName?: (isRightAlign: boolean) => string;
  onToggle?: (visible: boolean) => void;
  children: (props: ChildrenProps) => ComponentChildren;
};

type MenuPosition = {
  top: number;
  left?: number;
  right?: number;
};

const DropdownButton = ({
  name,
  title,
  className,
  buildDropdownClassName,
  onToggle,
  children,
}: Props) => {
  const [visible, setVisible] = useState(false);
  const [isRightAlign, setIsRightAlign] = useState(false);
  const [menuPos, setMenuPos] = useState<MenuPosition>({ top: 0, left: 0 });

  /** Ref on the trigger button — used to compute the portal's fixed coords. */
  const buttonRef = useRef<HTMLButtonElement>(null);
  /** Ref on the portaled menu div — used for click-outside detection. */
  const menuRef = useRef<HTMLDivElement>(null);

  /** Compute fixed position and alignment from the button's current rect. */
  const computePositionAndAlignment = useCallback(() => {
    const btn = buttonRef.current;
    if (!btn) return null;

    const rect = btn.getBoundingClientRect();
    const rightAligned = rect.x + rect.width / 2 > window.innerWidth / 2;

    const pos: MenuPosition = { top: rect.bottom + 8 };
    if (rightAligned) {
      pos.right = window.innerWidth - rect.right;
    } else {
      pos.left = rect.left;
    }

    return { pos, rightAligned };
  }, []);

  const toggleDropdown = useCallback(() => {
    if (visible) {
      setVisible(false);
      return;
    }

    const result = computePositionAndAlignment();
    if (result) {
      setMenuPos(result.pos);
      setIsRightAlign(result.rightAligned);
    }
    setVisible(true);
  }, [visible, computePositionAndAlignment]);

  /** When the menu is open, wire up dismiss events. */
  useEffect(() => {
    if (!visible) return;

    const handlePointerDown = (e: MouseEvent) => {
      const target = e.target as Node;
      if (
        !buttonRef.current?.contains(target) &&
        !menuRef.current?.contains(target)
      ) {
        setVisible(false);
      }
    };

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') setVisible(false);
    };

    // Close (and don't reposition) when the page scrolls or the window resizes,
    // since a fixed menu won't follow the button after those events.
    const handleScrollOrResize = () => setVisible(false);

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
  }, [visible]);

  useEffect(() => onToggle && onToggle(visible), [onToggle, visible]);

  const containerClassNames = ['dropdown-container', className]
    .filter(Boolean)
    .join(' ');

  const dropdownClassNames = [
    'dropdown-menu',
    buildDropdownClassName && buildDropdownClassName(isRightAlign),
  ]
    .filter(Boolean)
    .join(' ');

  const menuStyle: Record<string, string | number> = {
    position: 'fixed',
    top: menuPos.top,
    zIndex: 50,
  };
  if (menuPos.right !== undefined) {
    menuStyle.right = menuPos.right;
  } else {
    menuStyle.left = menuPos.left ?? 0;
  }

  return (
    <div className={containerClassNames}>
      <button
        id={`${name}-button`}
        className="btn"
        ref={buttonRef}
        onClick={toggleDropdown}
      >
        {title}
      </button>
      {visible &&
        createPortal(
          <div
            ref={menuRef}
            aria-labelledby={`${name}-button`}
            aria-orientation="vertical"
            className={dropdownClassNames}
            role="menu"
            tabIndex={-1}
            style={menuStyle}
          >
            {children({ dismiss: () => setVisible(false) })}
          </div>,
          document.body
        )}
    </div>
  );
};

export default DropdownButton;
