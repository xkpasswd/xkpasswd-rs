import init, * as xkpasswd from 'xkpasswd';

async function run() {
  await init();

  const pass = new xkpasswd.Xkpasswd();

  try {
    const customSettings = new xkpasswd.Settings()
      .withWordsCount(3)
      .withWordLengths(null, 8)
      .withSeparators('.')
      .withPaddingDigits(null, 2)
      .withPaddingSymbols('!@#$%^&*-_=+:|~?/;')
      .withPaddingSymbolLengths(null, 2)
      .withWordTransforms(
        xkpasswd.WordTransform.Lowercase,
        xkpasswd.WordTransform.Uppercase
      );

    appendPasswd('Custom', pass.genPass(customSettings));
  } catch (exc) {
    console.warn(exc);
  }

  const presetTitles = [
    'AppleID',
    'Default',
    'WindowsNTLMv1',
    'SecurityQuestions',
    'Web16',
    'Web32',
    'Wifi',
    'XKCD',
  ];

  [
    xkpasswd.Preset.AppleID,
    xkpasswd.Preset.Default,
    xkpasswd.Preset.WindowsNtlmV1,
    xkpasswd.Preset.SecurityQuestions,
    xkpasswd.Preset.Web16,
    xkpasswd.Preset.Web32,
    xkpasswd.Preset.Wifi,
    xkpasswd.Preset.Xkcd,
  ].forEach((preset, idx) => {
    const settings = xkpasswd.Settings.fromPreset(preset);
    appendPasswd(presetTitles[idx], pass.genPass(settings));
  });
}

function appendPasswd(title, passwd) {
  const div = document.createElement('div');

  const span = document.createElement('span');
  span.textContent = `${title}:`;
  span.style = 'color: darkgray; font-weight: bold;';
  div.appendChild(span);

  const pre = document.createElement('pre');
  pre.textContent = passwd;
  pre.style = 'display: inline; font-weight: bold; margin-left: 0.5em;';
  div.appendChild(pre);

  document.body.append(div);
}

run();
