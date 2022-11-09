import init, * as xkpasswd from 'xkpasswd';

async function run() {
  await init();

  const pre = document.createElement('pre');
  const settings = new xkpasswd.Settings(3);
  pre.textContent = xkpasswd.gen_pass(settings);
  document.body.append(pre);
}

run();
