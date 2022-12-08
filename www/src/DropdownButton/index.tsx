import { ComponentChildren } from 'preact';
import { useCallback, useEffect, useRef, useState } from 'preact/hooks';
import './styles.css';

const isVisible = (elem: HTMLElement | null) =>
  !!elem &&
  !!(elem.offsetWidth || elem.offsetHeight || elem.getClientRects().length);

type ChildrenProps = {
  dismiss: () => void;
};

type Props = {
  name: string;
  title: string;
  className?: string;
  buildDropdownClassName?: (isRightAlign: boolean) => string;
  children: (props: ChildrenProps) => ComponentChildren;
};

const DropdownButton = ({
  name,
  title,
  className,
  buildDropdownClassName,
  children,
}: Props) => {
  const [visible, setVisible] = useState(false);
  const [isRightAlign, setIsRightAlign] = useState(false);

  const ref = useRef<HTMLDivElement>(null);

  const setDropdownAlignment = useCallback(() => {
    const selfRef = ref.current;
    if (!selfRef) {
      return;
    }

    const { x, width } = selfRef.getBoundingClientRect();
    setIsRightAlign(x + width / 2 > window.screen.width / 2);
  }, [setIsRightAlign]);

  const toggleDropdown = useCallback(() => {
    if (visible) {
      setVisible(false);
      return;
    }

    setDropdownAlignment();
    setVisible(true);
  }, [visible, setVisible, setDropdownAlignment]);

  useEffect(() => {
    const outsideClickListener = (event: MouseEvent) => {
      if (
        !ref.current?.contains(event.target as HTMLElement) &&
        isVisible(ref.current)
      ) {
        setVisible(false);
      }
    };

    document.addEventListener('click', outsideClickListener);
    return () => document.removeEventListener('click', outsideClickListener);
  }, []);

  const containerClassNames = ['dropdown-container', className]
    .filter(Boolean)
    .join(' ');

  const dropdownClassNames = [
    'dropdown-menu',
    buildDropdownClassName && buildDropdownClassName(isRightAlign),
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <div className={containerClassNames} ref={ref}>
      <button id={`${name}-button`} className="btn" onClick={toggleDropdown}>
        {title}
      </button>
      {visible && (
        <div
          aria-labelledby={`${name}-button`}
          aria-orientation="vertical"
          className={dropdownClassNames}
          role="menu"
          tabIndex={-1}
        >
          {children({ dismiss: () => setVisible(false) })}
        </div>
      )}
    </div>
  );
};

export default DropdownButton;
