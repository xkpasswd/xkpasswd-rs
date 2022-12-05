import * as xkpasswd from '../../xkpasswd/xkpasswd';
import './styles.css';
import feelsGoodImage from '/feels-good.png';
import notBadImage from '/not-bad.png';
import rageFaceImage from '/rage-face.png';

const NOT_BAD_ENTROPY_BLIND = 78;
const NOT_BAD_ENTROPY_SEEN = 52;
const GREAT_ENTROPY_BLIND = NOT_BAD_ENTROPY_BLIND * 1.5;
const GREAT_ENTROPY_SEEN = NOT_BAD_ENTROPY_SEEN * 1.5;

type Props = {
  entropy?: xkpasswd.Entropy;
};

type BitsProps = {
  value: number;
  seen?: boolean;
};

const Bits = ({ value, seen = false }: BitsProps) => {
  const safeEntropy = seen ? NOT_BAD_ENTROPY_SEEN : NOT_BAD_ENTROPY_BLIND;
  const classNames =
    value >= safeEntropy ? 'entropy entropy-good' : 'entropy entropy-bad';
  return <span className={classNames}>{value}</span>;
};

const Entropy = ({ entropy }: Props) => {
  if (!entropy) {
    return null;
  }

  const entropyBlindMin = <Bits value={entropy.blind_min} />;
  const entropyBlindMax = <Bits value={entropy.blind_max} />;

  const entropyBlind =
    entropy.blind_min == entropy.blind_max ? (
      <>
        {`Btw, it's entropy is of `}
        {entropyBlindMin}
        {' bits blind & '}
      </>
    ) : (
      <>
        {`Btw, it's entropy is between `}
        {entropyBlindMin}
        {' and '}
        {entropyBlindMax}
        {' bits blind & '}
      </>
    );

  return (
    <div className="section">
      <span>
        {entropyBlind}
        <Bits seen value={entropy.seen} />
        {' bits with full knowledge, '}
        <Ratings entropy={entropy} />
      </span>
      <span className="entropy-recommendation">
        {`(It's recommended to keep `}
        <Bits value={NOT_BAD_ENTROPY_BLIND} />
        {' bits blind & '}
        <Bits seen value={NOT_BAD_ENTROPY_SEEN} />
        {' bits with full knowledge.)'}
      </span>
    </div>
  );
};

const Ratings = ({ entropy }: Props) => {
  if (!entropy) {
    return null;
  }

  if (
    entropy.blind_min < NOT_BAD_ENTROPY_BLIND ||
    entropy.seen < NOT_BAD_ENTROPY_SEEN
  ) {
    return (
      <>
        {'which is not good!'}
        <img className="entropy-img rage-face" src={rageFaceImage} />
      </>
    );
  }

  if (
    entropy.blind_min >= GREAT_ENTROPY_BLIND &&
    entropy.seen >= GREAT_ENTROPY_SEEN
  ) {
    return (
      <>
        {'which is great!'}
        <img className="entropy-img feels-good" src={feelsGoodImage} />
      </>
    );
  }

  return (
    <>
      {'which is not bad!'}
      <img className="entropy-img not-bad" src={notBadImage} />
    </>
  );
};

export default Entropy;
