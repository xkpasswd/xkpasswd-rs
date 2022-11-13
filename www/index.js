import init, * as xkpasswd from 'xkpasswd';

async function run() {
  await init();

  const pass = new xkpasswd.Xkpasswd();
  const settings = xkpasswd.Settings.default()
    .setWordsCount(3)
    .setWordLengths(4, 8)
    .setSeparators('._~')
    .setPaddingDigits(0, 2);

  Array(10)
    .fill(0)
    .forEach((_) => {
      const pre = document.createElement('pre');
      pre.textContent = pass.genPass(settings);
      document.body.append(pre);
    });
}

run();
