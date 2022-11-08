import init, * as xkpasswd from 'xkpasswd';

async function run() {
  await init();
  const pre = document.createElement('pre');
  pre.textContent = xkpasswd.gen_passwd(3);
  document.body.append(pre);
}

run();
