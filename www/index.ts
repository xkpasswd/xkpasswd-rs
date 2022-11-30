import init, * as xkpasswd from 'xkpasswd';

const PRESET_MAPPER: Record<string, xkpasswd.Preset> = {
  default: xkpasswd.Preset.Default,
  appleId: xkpasswd.Preset.AppleID,
  ntlm: xkpasswd.Preset.WindowsNtlmV1,
  secq: xkpasswd.Preset.SecurityQuestions,
  web16: xkpasswd.Preset.Web16,
  web32: xkpasswd.Preset.Web32,
  wifi: xkpasswd.Preset.Wifi,
  xkcd: xkpasswd.Preset.Xkcd,
};

async function bootstrap() {
  await init();
  const pass = new xkpasswd.Xkpasswd();

  document
    .querySelector('#presets-select')
    ?.addEventListener('change', (event) => {
      const target = event.target as HTMLSelectElement;
      console.log('target', target.innerText);
      const preset = PRESET_MAPPER[target.value];
      const settings = xkpasswd.Settings.fromPreset(preset);
      appendPasswd('Preset', pass.genPass(settings));
    });
}

function appendPasswd(title: string, passwd: string) {
  const div = document.createElement('div');

  const span = document.createElement('span');
  span.textContent = `${title}:`;
  span.className = 'font-bold text-slate-400 mr-1';
  div.appendChild(span);

  const pre = document.createElement('pre');
  pre.textContent = passwd;
  pre.className = 'inline-block font-bold font-mono hover:underline';
  div.appendChild(pre);

  document.body.append(div);
}

bootstrap();
