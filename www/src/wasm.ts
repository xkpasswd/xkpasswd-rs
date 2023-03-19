import initWasm, * as xkpasswd from '../xkpasswd/xkpasswd-en';

export async function lazyLoad() {
  const params = new URLSearchParams(window.location.search);
  const lang = params.get('lang');
  return lang;
}

await initWasm();
export default xkpasswd;
