import { describe, it, expect } from 'vitest';
import { cmdString } from './cmdString';

// ── helpers ────────────────────────────────────────────────────────────────

const BASE_BUILDER = {
  preset: undefined,
  wordsCount: 3,
  wordTransforms: 5, // Lowercase(1) | Uppercase(4)
  separators: '.',
  digitsBefore: 0,
  digitsAfter: 2,
  symbolsBefore: 1,
  symbolsAfter: 1,
  paddingSymbols: '~@$%^&*-_+=:|?/.;',
  adaptivePadding: false,
  adaptiveCount: 32,
};

// ── named preset ───────────────────────────────────────────────────────────

describe('cmdString – named preset', () => {
  it('emits only --preset and --lang for a named preset', () => {
    const cmd = cmdString({ ...BASE_BUILDER, preset: 0 }, 'en');
    expect(cmd).toBe('xkpasswd --preset=default --lang=en');
  });

  it('uses correct CLI name for each preset', () => {
    const cases: Array<[number, string]> = [
      [0, 'default'],
      [1, 'apple-id'],
      [2, 'ntlm'],
      [3, 'secq'],
      [4, 'web16'],
      [5, 'web32'],
      [6, 'wifi'],
      [7, 'xkcd'],
    ];
    for (const [preset, name] of cases) {
      const cmd = cmdString({ ...BASE_BUILDER, preset }, 'en');
      expect(cmd).toBe(`xkpasswd --preset=${name} --lang=en`);
    }
  });

  it('never emits --preset=custom', () => {
    // preset=undefined → custom mode, no --preset flag at all
    const cmd = cmdString({ ...BASE_BUILDER, preset: undefined }, 'en');
    expect(cmd).not.toContain('--preset');
    expect(cmd).not.toContain('--preset=custom');
  });

  it('uses the lang parameter', () => {
    const cmd = cmdString({ ...BASE_BUILDER, preset: 7 }, 'fr');
    expect(cmd).toBe('xkpasswd --preset=xkcd --lang=fr');
  });
});

// ── custom fixed padding ───────────────────────────────────────────────────

describe('cmdString – custom fixed padding', () => {
  it('emits all required flags in canonical order', () => {
    const cmd = cmdString(BASE_BUILDER, 'en');
    // Must start with xkpasswd
    expect(cmd).toMatch(/^xkpasswd /);
    // Must contain each required flag
    expect(cmd).toContain('--lang=en');
    expect(cmd).toContain('--words=3');
    expect(cmd).toContain('--separators="."');
    expect(cmd).toContain('--padding=fixed');
    expect(cmd).toContain('--digits-before=0');
    expect(cmd).toContain('--digits-after=2');
    expect(cmd).toContain(`--symbols="${BASE_BUILDER.paddingSymbols}"`);
    expect(cmd).toContain('--symbols-before=1');
    expect(cmd).toContain('--symbols-after=1');
    // No preset
    expect(cmd).not.toContain('--preset');
    // No adaptive flags
    expect(cmd).not.toContain('--adaptive-length');
  });

  it('emits --transforms flags for each active bit', () => {
    // wordTransforms=5 = Lowercase(1) | Uppercase(4)
    const cmd = cmdString(BASE_BUILDER, 'en');
    expect(cmd).toContain('--transforms=lowercase');
    expect(cmd).toContain('--transforms=uppercase');
    // Titlecase(2) not set
    expect(cmd).not.toContain('--transforms=titlecase');
  });

  it('emits no --transforms flags when wordTransforms=0', () => {
    const cmd = cmdString({ ...BASE_BUILDER, wordTransforms: 0 }, 'en');
    expect(cmd).not.toContain('--transforms');
  });

  it('canonical flag order: lang, words, separators, transforms, padding, digits, symbols, symbol-lengths', () => {
    const cmd = cmdString(BASE_BUILDER, 'en');
    const langIdx = cmd.indexOf('--lang');
    const wordsIdx = cmd.indexOf('--words');
    const sepsIdx = cmd.indexOf('--separators');
    const paddingIdx = cmd.indexOf('--padding');
    const digitBeforeIdx = cmd.indexOf('--digits-before');
    const digitAfterIdx = cmd.indexOf('--digits-after');
    const symbolsIdx = cmd.indexOf('--symbols=');
    const symbolsBeforeIdx = cmd.indexOf('--symbols-before');
    const symbolsAfterIdx = cmd.indexOf('--symbols-after');
    expect(langIdx).toBeLessThan(wordsIdx);
    expect(wordsIdx).toBeLessThan(sepsIdx);
    expect(sepsIdx).toBeLessThan(paddingIdx);
    expect(paddingIdx).toBeLessThan(digitBeforeIdx);
    expect(digitBeforeIdx).toBeLessThan(digitAfterIdx);
    expect(digitAfterIdx).toBeLessThan(symbolsIdx);
    expect(symbolsIdx).toBeLessThan(symbolsBeforeIdx);
    expect(symbolsBeforeIdx).toBeLessThan(symbolsAfterIdx);
  });
});

// ── custom adaptive padding ────────────────────────────────────────────────

describe('cmdString – custom adaptive padding', () => {
  const adaptiveBuilder = {
    ...BASE_BUILDER,
    adaptivePadding: true,
    adaptiveCount: 32,
  };

  it('emits --padding=adaptive and --adaptive-length', () => {
    const cmd = cmdString(adaptiveBuilder, 'de');
    expect(cmd).toContain('--padding=adaptive');
    expect(cmd).toContain('--adaptive-length=32');
    expect(cmd).not.toContain('--symbols-before');
    expect(cmd).not.toContain('--symbols-after');
  });

  it('does not emit symbol count flags in adaptive mode', () => {
    const cmd = cmdString(adaptiveBuilder, 'en');
    expect(cmd).not.toContain('--symbols-before');
    expect(cmd).not.toContain('--symbols-after');
  });

  it('does not emit --preset in adaptive custom mode', () => {
    const cmd = cmdString(adaptiveBuilder, 'en');
    expect(cmd).not.toContain('--preset');
  });
});

// ── lang parameter variants ────────────────────────────────────────────────

describe('cmdString – lang variants', () => {
  const langs = ['en', 'fr', 'de', 'pt', 'es'];
  for (const lang of langs) {
    it(`correctly uses lang=${lang}`, () => {
      const cmd = cmdString(BASE_BUILDER, lang);
      expect(cmd).toContain(`--lang=${lang}`);
    });
  }
});

// ── special character escaping in pool flags ───────────────────────────────

describe('cmdString – pool flag escaping', () => {
  it('escapes double-quote and backslash in --separators', () => {
    // separators = a"b\c  (actual chars: a, ", b, \, c)
    const cmd = cmdString({ ...BASE_BUILDER, separators: 'a"b\\c' }, 'en');
    expect(cmd).toContain('--separators="a\\"b\\\\c"');
  });

  it('escapes double-quote and backslash in --symbols', () => {
    // paddingSymbols = a"b\c  (actual chars: a, ", b, \, c)
    const cmd = cmdString({ ...BASE_BUILDER, paddingSymbols: 'a"b\\c' }, 'en');
    expect(cmd).toContain('--symbols="a\\"b\\\\c"');
  });

  it('leaves pool values without special chars unchanged', () => {
    // BASE_BUILDER uses safe chars — existing pools must still match verbatim
    const cmd = cmdString(BASE_BUILDER, 'en');
    expect(cmd).toContain(`--separators="${BASE_BUILDER.separators}"`);
    expect(cmd).toContain(`--symbols="${BASE_BUILDER.paddingSymbols}"`);
  });
});

// ── all named presets verified ─────────────────────────────────────────────

describe('cmdString – all transform flags', () => {
  it('Titlecase only (bit 2)', () => {
    const cmd = cmdString({ ...BASE_BUILDER, wordTransforms: 2 }, 'en');
    expect(cmd).toContain('--transforms=titlecase');
    expect(cmd).not.toContain('--transforms=lowercase');
    expect(cmd).not.toContain('--transforms=uppercase');
  });

  it('InversedTitlecase only (bit 8)', () => {
    const cmd = cmdString({ ...BASE_BUILDER, wordTransforms: 8 }, 'en');
    expect(cmd).toContain('--transforms=inversed-titlecase');
  });

  it('AltercaseLowerFirst only (bit 64)', () => {
    const cmd = cmdString({ ...BASE_BUILDER, wordTransforms: 64 }, 'en');
    expect(cmd).toContain('--transforms=altercase-lower-first');
  });

  it('AltercaseUpperFirst only (bit 128)', () => {
    const cmd = cmdString({ ...BASE_BUILDER, wordTransforms: 128 }, 'en');
    expect(cmd).toContain('--transforms=altercase-upper-first');
  });
});
