import passwordImage from '/logo-64x64.png';
import './styles.css';

const Header = () => {
  return (
    <div className="header-container">
      <img className="header-logo" src={passwordImage} />
      <h1 className="header-title">{'XKCD Password Generator'}</h1>
    </div>
  );
};

export default Header;
