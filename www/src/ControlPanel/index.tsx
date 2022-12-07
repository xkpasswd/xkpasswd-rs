import Presets from './Presets';
import './styles.css';

type Props = {
  onGenerate: () => void;
};

const ControlPanel = ({ onGenerate }: Props) => (
  <div className="section settings">
    <span>
      {'Hey, can you please '}
      <button className="btn btn-generate" onClick={onGenerate}>
        {'generate'}
      </button>
      {' a password using '}
      <Presets />
      {' preset?'}
    </span>
  </div>
);

export default ControlPanel;
