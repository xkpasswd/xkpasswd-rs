import * as xkpasswd from '../../xkpasswd/xkpasswd';
import './styles.css';

type Props = {
  entropy?: xkpasswd.Entropy;
};

const Entropy = ({ entropy }: Props) => {
  if (!entropy) {
    return null;
  }

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

  const entropyBlind =
    entropy.blind_min == entropy.blind_max
      ? [`Btw, it's entropy is of `, entropyBlindMin, ' bits blind & ']
      : [
          `Btw, it's entropy is between `,
          entropyBlindMin,
          ' and ',
          entropyBlindMax,
          ' bits blind & ',
        ];

  const entropySeen = [
    <span className="entropy" key="entropy-seen">
      {entropy.seen}
    </span>,
    ' bits with full knowledge!',
  ];

  return (
    <div className="mt-4">
      <span>{[...entropyBlind, ...entropySeen]}</span>
    </div>
  );
};

export default Entropy;
