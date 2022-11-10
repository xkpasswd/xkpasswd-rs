import init, * as xkpasswd from 'xkpasswd';

async function run() {
  await init();

  const pre = document.createElement('pre');
  pre.textContent = xkpasswd.gen_pass({
    words_count: 3,
    word_lengths: [3, 5],
  });
  document.body.append(pre);
}

run();
