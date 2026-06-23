/**
 * Builds a copyable `xkpasswd …` command string from the current settings.
 *
 * Named preset → `xkpasswd --preset=<name> --lang=<lang>` only.
 * Custom       → full flag list (no --preset flag).
 */
import { PRESET_CLI_NAME, TRANSFORM_CLI_NAME } from './cliNames';

/**
 * Minimal builder shape — structurally compatible with SettingsBuilderType
 * from contexts.tsx.
 */
type BuilderShape = Readonly<{
  preset?: number;
  wordsCount: number;
  wordTransforms: number;
  separators: string;
  digitsBefore: number;
  digitsAfter: number;
  symbolsBefore: number;
  symbolsAfter: number;
  paddingSymbols: string;
  adaptivePadding: boolean;
  adaptiveCount: number;
}>;

/**
 * Build the active WordTransform CLI flags from a bitfield.
 * Returns one `--transforms=<name>` token per set bit.
 */
function transformFlags(wordTransforms: number): string[] {
  return Object.entries(TRANSFORM_CLI_NAME)
    .filter(([bit]) => (wordTransforms & Number(bit)) !== 0)
    .map(([, name]) => `--transforms=${name}`);
}

/**
 * Escapes backslashes and double-quotes so a pool value is safe inside a
 * double-quoted shell argument: `\` → `\\`, `"` → `\"`.
 * Backslash must be escaped first to avoid double-escaping.
 */
const dq = (s: string): string =>
  s.replace(/\\/g, '\\\\').replace(/"/g, '\\"');

/**
 * Returns a ready-to-paste `xkpasswd …` command string.
 *
 * - Named preset:   `xkpasswd --preset=<name> --lang=<lang>`
 * - Custom settings: full flag list in canonical order; `--preset` is NEVER
 *   emitted (absence signals custom mode to the real CLI).
 */
export function cmdString(builder: BuilderShape, lang: string): string {
  if (builder.preset != null) {
    const name = PRESET_CLI_NAME[builder.preset] ?? 'default';
    return `xkpasswd --preset=${name} --lang=${lang}`;
  }

  const parts: string[] = ['xkpasswd'];

  parts.push(`--lang=${lang}`);
  parts.push(`--words=${builder.wordsCount}`);
  parts.push(`--separators="${dq(builder.separators)}"`);

  // One --transforms flag per active bit, in TRANSFORM_CLI_NAME iteration order
  parts.push(...transformFlags(builder.wordTransforms));

  parts.push(`--padding=${builder.adaptivePadding ? 'adaptive' : 'fixed'}`);
  parts.push(`--digits-before=${builder.digitsBefore}`);
  parts.push(`--digits-after=${builder.digitsAfter}`);
  parts.push(`--symbols="${dq(builder.paddingSymbols)}"`);

  if (builder.adaptivePadding) {
    parts.push(`--adaptive-length=${builder.adaptiveCount}`);
  } else {
    parts.push(`--symbols-before=${builder.symbolsBefore}`);
    parts.push(`--symbols-after=${builder.symbolsAfter}`);
  }

  return parts.join(' ');
}
