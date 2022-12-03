import * as xkpasswd from '../../xkpasswd/xkpasswd';
import './styles.css';

type Props = {
  entropy?: xkpasswd.Entropy;
};

const Entropy = ({ entropy }: Props) => {
  if (!entropy) {
    return null;
  }

  let content = [];
  const entropyBlindMin = (
    <span className="entropy" key="entropy-blind-min">
      {entropy.blind_min}
    </span>
  );
  const entropyBlindMax = (
    <span className="entropy" key="entropy-blind-max">
      {entropy.blind_max}
    </span>
  );
  const entropySeen = (
    <span className="entropy" key="entropy-seen">
      {entropy.seen}
    </span>
  );

  if (entropy.blind_min == entropy.blind_max) {
    content = [
      'Entropy is of ',
      entropyBlindMin,
      ' bits blind & ',
      entropySeen,
      ' bits with full knowledge',
    ];
  } else {
    content = [
      'Entropy is between ',
      entropyBlindMin,
      ' and ',
      entropyBlindMax,
      ' bits blind & ',
      entropySeen,
      ' bits with full knowledge',
    ];
  }

  return (
    <div className="mt-8">
      <span className="text-gray">{content}</span>
    </div>
  );
};

export default Entropy;
