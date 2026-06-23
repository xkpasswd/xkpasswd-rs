/**
 * CLI name maps for xkpasswd command-line arguments.
 *
 * Preset values match xkpasswd.Preset enum:
 *   Default=0, AppleID=1, WindowsNtlmV1=2, SecurityQuestions=3,
 *   Web16=4, Web32=5, Wifi=6, Xkcd=7
 *
 * WordTransform values are bit-flags:
 *   Lowercase=1, Titlecase=2, Uppercase=4, InversedTitlecase=8,
 *   AltercaseLowerFirst=64, AltercaseUpperFirst=128
 */

export const PRESET_CLI_NAME: Readonly<Record<number, string>> = {
  0: 'default',     // Preset.Default
  1: 'apple-id',    // Preset.AppleID
  2: 'ntlm',        // Preset.WindowsNtlmV1
  3: 'secq',        // Preset.SecurityQuestions
  4: 'web16',       // Preset.Web16
  5: 'web32',       // Preset.Web32
  6: 'wifi',        // Preset.Wifi
  7: 'xkcd',        // Preset.Xkcd
};

export const TRANSFORM_CLI_NAME: Readonly<Record<number, string>> = {
  1: 'lowercase',              // WordTransform.Lowercase
  2: 'titlecase',              // WordTransform.Titlecase
  4: 'uppercase',              // WordTransform.Uppercase
  8: 'inversed-titlecase',     // WordTransform.InversedTitlecase
  64: 'altercase-lower-first', // WordTransform.AltercaseLowerFirst
  128: 'altercase-upper-first', // WordTransform.AltercaseUpperFirst
};
