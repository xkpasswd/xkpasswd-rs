import * as xkpasswd from '../../xkpasswd/xkpasswd';
import './styles.css';
import notBadImage from '/not-bad.png';
import rageFaceImage from '/rage-face.png';

const MIN_RECOMMENDED_ENTROPY_BLIND = 78;
const MIN_RECOMMENDED_ENTROPY_SEEN = 52;

type Props = {
  entropy?: xkpasswd.Entropy;
};

const entropyClassName = (good: boolean) =>
  good ? 'entropy entropy-good' : 'entropy entropy-bad';

const Entropy = ({ entropy }: Props) => {
  if (!entropy) {
    return null;
  }

  const entropyBlindMin = (
    <span
      className={entropyClassName(
        entropy.blind_min >= MIN_RECOMMENDED_ENTROPY_BLIND
      )}
      key="entropy-blind-min"
    >
      {entropy.blind_min}
    </span>
  );

  const entropyBlindMax = (
    <span
      className={entropyClassName(
        entropy.blind_max >= MIN_RECOMMENDED_ENTROPY_BLIND
      )}
      key="entropy-blind-max"
    >
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
    <span
      className={entropyClassName(entropy.seen >= MIN_RECOMMENDED_ENTROPY_SEEN)}
      key="entropy-seen"
    >
      {entropy.seen}
    </span>,
    ' bits with full knowledge, ',
  ];

  const recommendation =
    entropy.blind_min >= MIN_RECOMMENDED_ENTROPY_BLIND &&
    entropy.seen >= MIN_RECOMMENDED_ENTROPY_SEEN
      ? [
          'which is not bad!',
          <img
            className="entropy-img not-bad"
            key="entropy-not-bad"
            src={notBadImage}
          />,
        ]
      : [
          'which is not good!',
          <img
            className="entropy-img rage-face"
            key="entropy-rage-face"
            src={rageFaceImage}
          />,
        ];

  return (
    <div className="mt-4">
      <span>{[...entropyBlind, ...entropySeen, ...recommendation]}</span>
    </div>
  );
};

export default Entropy;
