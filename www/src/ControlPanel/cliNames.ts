/**
 * CLI name maps for xkpasswd command-line arguments.
 *
 * Keyed by the numeric values of the WASM-exported enums. We intentionally do
 * NOT import the `xkpasswd` runtime here: `src/wasm` touches `window` and runs
 * a top-level `await initWasm()`, so importing it would break the node-based
 * vitest suite that exercises `cmdString` (which imports this module).
 *
 * Keys are the numeric enum values (kept in sync with the Rust source of truth
 * below). The consuming UI uses the `xkpasswd.Preset.*` / `xkpasswd.WordTransform.*`
 * enums directly; this module only translates their numeric values to CLI names.
 *
 * Preset values:  Default=0, AppleID=1, WindowsNtlmV1=2, SecurityQuestions=3,
 *                 Web16=4, Web32=5, Wifi=6, Xkcd=7
 * WordTransform values (bit-flags): Lowercase=1, Titlecase=2, Uppercase=4,
 *                 InversedTitlecase=8, AltercaseLowerFirst=64, AltercaseUpperFirst=128
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
