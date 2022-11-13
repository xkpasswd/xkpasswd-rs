import init, * as xkpasswd from 'xkpasswd';

async function run() {
  await init();

  const pass = new xkpasswd.Xkpasswd();

  Array(10)
    .fill(0)
    .forEach((_) => {
      const passwd = pass.gen_pass({
        words_count: 3,
        word_lengths: [4, 8],
        separators: '._~'.split(''),
      });

      const pre = document.createElement('pre');
      pre.textContent = passwd;
      document.body.append(pre);
    });
}

run();
