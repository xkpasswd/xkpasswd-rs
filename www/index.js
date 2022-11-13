import init, * as xkpasswd from 'xkpasswd';

async function run() {
  await init();

  const pass = new xkpasswd.Xkpasswd();
  const settings = xkpasswd.Settings.default()
    .withWordsCount(3)
    .withWordLengths(4, 8)
    .withSeparators('._-')
    .withPaddingDigits(0, 2)
    .withPaddingSymbols('!@#$%^&*-_=+:|~?/;')
    .withPaddingSymbolLengths(0, 2);

  Array(10)
    .fill(0)
    .forEach((_) => {
      const pre = document.createElement('pre');
      pre.textContent = pass.genPass(settings);
      document.body.append(pre);
    });
}

run();
