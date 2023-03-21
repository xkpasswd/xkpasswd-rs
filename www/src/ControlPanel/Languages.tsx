import DropdownButton from 'src/DropdownButton';
import { LANGUAGE_MAPPING, language } from 'src/wasm';
import './styles.css';

const LANGUAGE_OPTIONS = Object.entries(LANGUAGE_MAPPING)
  .sort()
  .map(([code, title]) => ({
    code,
    title,
  }));

const DEFAULT_OPTION = { code: 'en', title: 'English' };

const Languages = () => {
  const option =
    LANGUAGE_OPTIONS.find(({ code }) => language == code) || DEFAULT_OPTION;

  return (
    <DropdownButton
      name="languages"
      title={option.title}
      buildDropdownClassName={(isRightAlign) =>
        isRightAlign
          ? 'languages-dropdown right-align'
          : 'languages-dropdown left-align'
      }
    >
      {() =>
        LANGUAGE_OPTIONS.map(({ code, title }) => (
          <a
            className="language-option"
            key={`language_option_${code}`}
            href={`/?lang=${code}`}
          >
            {title}
          </a>
        ))
      }
    </DropdownButton>
  );
};

export default Languages;
