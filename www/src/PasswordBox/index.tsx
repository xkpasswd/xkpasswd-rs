import { useCallback, useEffect, useRef, useState } from 'preact/hooks';
import { copyToClipboard } from '../utils';
import './styles.css';

type Props = {
  passwd: string;
};

const PasswordBox = ({ passwd }: Props) => {
  const [showTooltips, setShowTooltips] = useState(false);

  const ref = useRef<HTMLButtonElement>(null);

  const copyPasswd = useCallback(() => {
    if (copyToClipboard(passwd)) {
      setShowTooltips(true);
    } else {
      prompt('Password to copy', passwd);
    }
  }, [passwd]);

  useEffect(() => {
    const blurEventHandler = () => setShowTooltips(false);
    const selfRef = ref.current;

    selfRef?.addEventListener('blur', blurEventHandler);
    return () => selfRef?.removeEventListener('blur', blurEventHandler);
  }, []);

  return (
    <>
      <div className="section">{'Sure, here you are (just tap to copy):'}</div>
      <div className="section password-box">
        <button className="btn-password" onClick={copyPasswd} ref={ref}>
          {passwd}
        </button>
        {showTooltips && (
          <div className="password-tooltips">{'Password copied'}</div>
        )}
      </div>
    </>
  );
};

export default PasswordBox;
