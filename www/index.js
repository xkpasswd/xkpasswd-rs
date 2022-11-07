import * as wasm from 'xkpasswd-rs';

const pre = document.createElement('pre');
pre.textContent = wasm.gen_passwd(3);
document.body.append(pre);
