import { InformationCircleIcon } from '@heroicons/react/24/outline';
import * as xkpasswd from '../../xkpasswd/xkpasswd';
import feelsGoodImage from '/feels-good.png';
import notBadImage from '/not-bad.png';
import rageFaceImage from '/rage-face.png';
import './styles.css';
import { pluralize } from '../utils';

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
        {'Btw, its entropy is of '}
        {entropyBlindMin}
        {' bits blind & '}
      </>
    ) : (
      <>
        {'Btw, its entropy is between '}
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
        <InformationCircleIcon className="information-icon" />
        {`It's recommended to keep `}
        <Bits value={NOT_BAD_ENTROPY_BLIND} />
        {' bits & '}
        <Bits seen value={NOT_BAD_ENTROPY_SEEN} />
        {' bits, respectively.'}
      </span>
    </div>
  );
};

const calcExceptionalTime = (years: number) => {
  if (years > 1_000_000_000) {
    return 'more than a billion years';
  }

  if (years > 1_000_000) {
    return 'more than a million years';
  }

  if (years > 1_000) {
    return 'more than a thousand years';
  }

  return null;
};

const GuessTime = ({ value }: { value: xkpasswd.GuessTime }) => {
  const prefix = 'which takes computer ';
  const suffix = ' to break at 1000 guesses/sec';
  const exceptionalTime = calcExceptionalTime(value.years);

  if (exceptionalTime) {
    return (
      <>
        {prefix}
        <span className="font-bold">{exceptionalTime}</span>
        {suffix}
      </>
    );
  }

  return (
    <>
      {prefix}
      {value.years > 0 && (
        <span className="font-bold">
          {value.years}
          {` ${pluralize(value.years, 'year')} `}
        </span>
      )}
      {value.months > 0 && (
        <span className="font-bold">
          {value.months}
          {` ${pluralize(value.months, 'month')} `}
        </span>
      )}
      {value.days > 0 && (
        <span className="font-bold">
          {value.days}
          {` ${pluralize(value.days, 'day')} `}
        </span>
      )}
      {suffix}
    </>
  );
};

const Ratings = ({ entropy }: { entropy: xkpasswd.Entropy }) => {
  if (
    entropy.blind_min < NOT_BAD_ENTROPY_BLIND ||
    entropy.seen < NOT_BAD_ENTROPY_SEEN
  ) {
    return (
      <>
        <GuessTime value={entropy.guess_time} />
        {'. Not good!'}
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
        <GuessTime value={entropy.guess_time} />
        {'. Great!'}
        <img className="entropy-img feels-good" src={feelsGoodImage} />
      </>
    );
  }

  return (
    <>
      <GuessTime value={entropy.guess_time} />
      {'. Not bad!'}
      <img className="entropy-img not-bad" src={notBadImage} />
    </>
  );
};

export default Entropy;
