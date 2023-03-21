const LANGUAGE_MAPPING = {
  en: 'English',
  fr: 'French',
  pt: 'Portuguese',
};

async function lazyLoad() {
  const params = new URLSearchParams(window.location.search);
  const language = params.get('lang');

  switch (language) {
    case 'fr': {
      const { default: initWasm, ...xkpasswd } = await import(
        '../xkpasswd/xkpasswd-fr'
      );

      return { initWasm, xkpasswd, language };
    }

    case 'pt': {
      const { default: initWasm, ...xkpasswd } = await import(
        '../xkpasswd/xkpasswd-pt'
      );

      return { initWasm, xkpasswd, language };
    }

    default: {
      const { default: initWasm, ...xkpasswd } = await import(
        '../xkpasswd/xkpasswd-en'
      );

      return { initWasm, xkpasswd, language: 'en' };
    }
  }
}

const { initWasm, xkpasswd, language } = await lazyLoad();

let wasmLoaded = false;

if (!wasmLoaded) {
  await initWasm();
  wasmLoaded = true;
}

export default xkpasswd;
export { LANGUAGE_MAPPING, language };
