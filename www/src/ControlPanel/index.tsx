/**
 * ControlPanel — command-line renderer driven by the shared SettingsBuilder.
 *
 * Renders an `xkpasswd …` command line where every flag value is an inline
 * editable control.  Output regenerates on COMMIT (blur/Enter) and also LIVE
 * per-keystroke via `regeneratePreview` (caret-safe — builder state is never
 * touched during typing, so token `value` props stay constant and caret survives).
 *
 * Layout rules (§5.3):
 *   - ≤2 args → single head line: `❯ xkpasswd <arg> <arg>`
 *   - >2 args → multi-line: head line `❯ xkpasswd \`, each arg on its own
 *     `.argline` with 4ch hanging indent, continuation `\` on every line but
 *     the last.
 *
 * NOTE: `--preset=custom` is displayed as a mode indicator but intentionally
 * omitted from the copied command (`cmdString` handles this — §7.1).
 */
import type { VNode } from 'preact';
import { useCallback, useEffect, useRef, useState } from 'preact/hooks';
import { useSettings } from 'src/contexts';
import xkpasswd, { LANGUAGE_MAPPING, language } from 'src/wasm';
import { PRESET_CLI_NAME, TRANSFORM_CLI_NAME } from './cliNames';
import {
  FIELD,
  activeTransforms,
  addTransform,
  cycleTransform,
  removeTransform,
} from './editing';
import { EditableNumber, EditableString } from './Editable';
import { cmdString } from './cmdString';
import { CopyIcon, RunIcon } from 'src/Icons';
import DropdownButton from 'src/DropdownButton';
import './styles.css';

// ── Static data (computed once at module load) ────────────────────────────────

/** All preset options in display order, keyed by CLI name. */
const PRESET_OPTIONS: ReadonlyArray<{
  label: string;
  preset: number | undefined;
}> = [
  { label: 'custom', preset: undefined },
  { label: 'default', preset: xkpasswd.Preset.Default },
  { label: 'apple-id', preset: xkpasswd.Preset.AppleID },
  { label: 'ntlm', preset: xkpasswd.Preset.WindowsNtlmV1 },
  { label: 'secq', preset: xkpasswd.Preset.SecurityQuestions },
  { label: 'web16', preset: xkpasswd.Preset.Web16 },
  { label: 'web32', preset: xkpasswd.Preset.Web32 },
  { label: 'wifi', preset: xkpasswd.Preset.Wifi },
  { label: 'xkcd', preset: xkpasswd.Preset.Xkcd },
];

/** Language codes sorted alphabetically, derived from WASM LANGUAGE_MAPPING. */
const LANGUAGE_OPTIONS: ReadonlyArray<string> = Object.keys(
  LANGUAGE_MAPPING
).sort();

// ── Fragment type ─────────────────────────────────────────────────────────────

interface ArgFragment {
  key: string;
  node: VNode;
}

// ── Inline helpers ────────────────────────────────────────────────────────────

/** `--flag=` rendered in muted colour (both parts are nowrap). */
const Flag = ({ name }: { name: string }) => (
  <span className="whitespace-nowrap">
    <span className="flag">--{name}</span>
    <span className="eq">=</span>
  </span>
);

/** Continuation backslash at end of an arg line. */
const Cont = () => <span className="cont"> \</span>;

// ── ControlPanel ──────────────────────────────────────────────────────────────

type Props = {
  onGenerate: () => void;
};

const ControlPanel = ({ onGenerate }: Props) => {
  const { builder, regeneratePreview } = useSettings();
  const [copied, setCopied] = useState(false);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // ── Derived state ──────────────────────────────────────────────────────────
  const isCustom = builder.preset == null;
  const presetName =
    builder.preset != null
      ? (PRESET_CLI_NAME[builder.preset] ?? 'default')
      : 'custom';
  const paddingValue = builder.adaptivePadding ? 'adaptive' : 'fixed';

  // Active transform bits in canonical order [1, 2, 4, 8, 64, 128].
  const transforms = activeTransforms(builder.wordTransforms);
  // Whether a new single transform can still be added.
  const canAddTransform =
    addTransform(builder.wordTransforms) !== builder.wordTransforms;

  // ── Callbacks ──────────────────────────────────────────────────────────────

  /**
   * Cycle --padding between fixed ⇄ adaptive.
   * Switching INTO adaptive also zeroes symbol counts (§7.2 exclusivity rule).
   */
  const handlePaddingCycle = useCallback(() => {
    if (!builder.adaptivePadding) {
      // Going to adaptive: zero the symbol counts so cmdString never emits them.
      builder.updateSymbolsBefore(0);
      builder.updateSymbolsAfter(0);
    }
    builder.toggleAdaptivePadding();
  }, [builder]);

  /**
   * Copy the current command to the clipboard.
   * Falls back to `execCommand` when the Clipboard API is unavailable.
   * Transiently swaps the label to "copied" for ~1.1 s.
   */
  const handleCopy = useCallback(async () => {
    const cmd = cmdString(builder, language);
    try {
      await navigator.clipboard.writeText(cmd);
    } catch {
      // Graceful fallback for insecure contexts.
      const ta = document.createElement('textarea');
      ta.value = cmd;
      ta.style.position = 'fixed';
      ta.style.opacity = '0';
      document.body.appendChild(ta);
      ta.select();
      document.execCommand('copy');
      document.body.removeChild(ta);
    }
    setCopied(true);
    if (timerRef.current) clearTimeout(timerRef.current);
    timerRef.current = setTimeout(() => setCopied(false), 1100);
  }, [builder]);

  // Clean up the timer on unmount to avoid state updates on dead components.
  useEffect(() => {
    return () => {
      if (timerRef.current) clearTimeout(timerRef.current);
    };
  }, []);

  // ── Build ordered arg list ─────────────────────────────────────────────────

  const args: ArgFragment[] = [];

  // ── 1. --preset (always) ───────────────────────────────────────────────────
  args.push({
    key: 'preset',
    node: (
      <span className="whitespace-nowrap">
        <Flag name="preset" />
        <DropdownButton
          name="presets"
          title={<span className="enum tok">{presetName}</span>}
          buildDropdownClassName={(isRight) =>
            isRight
              ? 'presets-dropdown right-align'
              : 'presets-dropdown left-align'
          }
        >
          {({ dismiss }) =>
            PRESET_OPTIONS.map(({ label, preset }) => (
              <button
                key={label}
                className="preset-option"
                onClick={() => {
                  builder.updatePreset(preset);
                  dismiss();
                }}
              >
                {label}
              </button>
            ))
          }
        </DropdownButton>
      </span>
    ),
  });

  // ── 2. --lang (always) ────────────────────────────────────────────────────
  args.push({
    key: 'lang',
    node: (
      <span className="whitespace-nowrap">
        <Flag name="lang" />
        <DropdownButton
          name="languages"
          title={<span className="enum tok">{language}</span>}
          buildDropdownClassName={(isRight) =>
            isRight
              ? 'languages-dropdown right-align'
              : 'languages-dropdown left-align'
          }
        >
          {() =>
            LANGUAGE_OPTIONS.map((code) => (
              <a key={code} className="language-option" href={`/?lang=${code}`}>
                {code}
              </a>
            ))
          }
        </DropdownButton>
      </span>
    ),
  });

  // ── Granular flags: custom mode only ──────────────────────────────────────
  if (isCustom) {
    // ── 3. --words ──────────────────────────────────────────────────────────
    args.push({
      key: 'words',
      node: (
        <span className="whitespace-nowrap">
          <Flag name="words" />
          <EditableNumber
            value={builder.wordsCount}
            min={FIELD.wordsCount.min}
            max={FIELD.wordsCount.max}
            emptyDefault={FIELD.wordsCount.default}
            onChange={builder.updateWordsCount}
            onLiveChange={(v) => regeneratePreview({ wordsCount: v })}
            className="num"
            ariaLabel="words count"
          />
        </span>
      ),
    });

    // ── 4. --separators ─────────────────────────────────────────────────────
    args.push({
      key: 'separators',
      node: (
        <span>
          <span className="whitespace-nowrap">
            <Flag name="separators" />
          </span>
          <span className="str">&quot;</span>
          <EditableString
            value={builder.separators}
            fallback="."
            onChange={builder.updateSeparators}
            onLiveChange={(v) => regeneratePreview({ separators: v })}
            className="str"
          />
          <span className="str">&quot;</span>
        </span>
      ),
    });

    // ── 5. --transforms (one per active bit) ─────────────────────────────────
    transforms.forEach((bit, i) => {
      const tfName = TRANSFORM_CLI_NAME[bit] ?? 'lowercase';
      const isLast = i === transforms.length - 1;

      const handleCycleTransform = () =>
        builder.updateWordTransforms(
          cycleTransform(builder.wordTransforms, i)
        );
      const handleRemoveTransform = () =>
        builder.updateWordTransforms(
          removeTransform(builder.wordTransforms, i)
        );
      const handleAddTransform = () =>
        builder.updateWordTransforms(addTransform(builder.wordTransforms));

      args.push({
        key: `transforms-${bit}`,
        node: (
          <span>
            <span className="whitespace-nowrap">
              <Flag name="transforms" />
              {/* Cycle the transform name on click/Enter/Space */}
              <span
                className="enum tok"
                tabIndex={0}
                onClick={handleCycleTransform}
                onKeyDown={(e: KeyboardEvent) => {
                  if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    handleCycleTransform();
                  }
                }}
              >
                {tfName}
              </span>
              {/* × remove: shown when more than one transform is active */}
              {transforms.length > 1 && (
                <button
                  className="tfbtn rm"
                  tabIndex={0}
                  title="remove transform"
                  onClick={handleRemoveTransform}
                  onKeyDown={(e: KeyboardEvent) => {
                    if (e.key === 'Enter' || e.key === ' ') {
                      e.preventDefault();
                      handleRemoveTransform();
                    }
                  }}
                >
                  {/* U+00D7 MULTIPLICATION SIGN — never ✕ */}
                  ×
                </button>
              )}
            </span>
            {/* + add: shown after the LAST transform, only when more can be added */}
            {isLast && canAddTransform && (
              <button
                className="tfbtn add"
                tabIndex={0}
                title="add transform"
                onClick={handleAddTransform}
                onKeyDown={(e: KeyboardEvent) => {
                  if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    handleAddTransform();
                  }
                }}
              >
                {/* U+002B PLUS SIGN — never ＋ */}
                +
              </button>
            )}
          </span>
        ),
      });
    });

    // ── 6. --padding (cycle fixed ⇄ adaptive) ──────────────────────────────
    args.push({
      key: 'padding',
      node: (
        <span className="whitespace-nowrap">
          <Flag name="padding" />
          <span
            className="enum tok"
            tabIndex={0}
            onClick={handlePaddingCycle}
            onKeyDown={(e: KeyboardEvent) => {
              if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                handlePaddingCycle();
              }
            }}
          >
            {paddingValue}
          </span>
        </span>
      ),
    });

    // ── 7. --digits-before ─────────────────────────────────────────────────
    args.push({
      key: 'digits-before',
      node: (
        <span className="whitespace-nowrap">
          <Flag name="digits-before" />
          <EditableNumber
            value={builder.digitsBefore}
            min={FIELD.digitsBefore.min}
            max={FIELD.digitsBefore.max}
            emptyDefault={FIELD.digitsBefore.default}
            onChange={builder.updateDigitsBefore}
            onLiveChange={(v) => regeneratePreview({ digitsBefore: v })}
            className="num"
            ariaLabel="digits before"
          />
        </span>
      ),
    });

    // ── 8. --digits-after ──────────────────────────────────────────────────
    args.push({
      key: 'digits-after',
      node: (
        <span className="whitespace-nowrap">
          <Flag name="digits-after" />
          <EditableNumber
            value={builder.digitsAfter}
            min={FIELD.digitsAfter.min}
            max={FIELD.digitsAfter.max}
            emptyDefault={FIELD.digitsAfter.default}
            onChange={builder.updateDigitsAfter}
            onLiveChange={(v) => regeneratePreview({ digitsAfter: v })}
            className="num"
            ariaLabel="digits after"
          />
        </span>
      ),
    });

    // ── 9. --symbols (pool — shown in both fixed and adaptive) ─────────────
    args.push({
      key: 'symbols',
      node: (
        <span>
          <span className="whitespace-nowrap">
            <Flag name="symbols" />
          </span>
          <span className="str">&quot;</span>
          <EditableString
            value={builder.paddingSymbols}
            fallback="%"
            onChange={builder.updatePaddingSymbols}
            onLiveChange={(v) => regeneratePreview({ paddingSymbols: v })}
            className="str"
          />
          <span className="str">&quot;</span>
        </span>
      ),
    });

    if (!builder.adaptivePadding) {
      // ── 9a. --symbols-before (fixed only) ────────────────────────────────
      args.push({
        key: 'symbols-before',
        node: (
          <span className="whitespace-nowrap">
            <Flag name="symbols-before" />
            <EditableNumber
              value={builder.symbolsBefore}
              min={FIELD.symbolsBefore.min}
              max={FIELD.symbolsBefore.max}
              emptyDefault={FIELD.symbolsBefore.default}
              onChange={builder.updateSymbolsBefore}
              onLiveChange={(v) => regeneratePreview({ symbolsBefore: v })}
              className="num"
              ariaLabel="symbols before"
            />
          </span>
        ),
      });

      // ── 9b. --symbols-after (fixed only) ─────────────────────────────────
      args.push({
        key: 'symbols-after',
        node: (
          <span className="whitespace-nowrap">
            <Flag name="symbols-after" />
            <EditableNumber
              value={builder.symbolsAfter}
              min={FIELD.symbolsAfter.min}
              max={FIELD.symbolsAfter.max}
              emptyDefault={FIELD.symbolsAfter.default}
              onChange={builder.updateSymbolsAfter}
              onLiveChange={(v) => regeneratePreview({ symbolsAfter: v })}
              className="num"
              ariaLabel="symbols after"
            />
          </span>
        ),
      });
    } else {
      // ── 9c. --adaptive-length (adaptive only) ────────────────────────────
      args.push({
        key: 'adaptive-length',
        node: (
          <span className="whitespace-nowrap">
            <Flag name="adaptive-length" />
            <EditableNumber
              value={builder.adaptiveCount}
              min={FIELD.adaptiveCount.min}
              max={FIELD.adaptiveCount.max}
              emptyDefault={FIELD.adaptiveCount.default}
              onChange={builder.updateAdaptiveCount}
              onLiveChange={(v) => regeneratePreview({ adaptiveCount: v })}
              className="num"
              ariaLabel="adaptive length"
            />
          </span>
        ),
      });
    }
  }

  // ── Render: single-line (≤2 args) or multi-line (>2 args) ─────────────────
  const isMultiLine = args.length > 2;

  return (
    <div className="section">
      <div className="cmd">
        {isMultiLine ? (
          <>
            {/* Head: `❯ xkpasswd \` */}
            <div className="argline head">
              <span className="prompt">❯</span>{' '}
              <span className="bin">xkpasswd</span>
              <Cont />
            </div>
            {/* One arg per line, all but the last get a trailing `\` */}
            {args.map(({ key, node }, i) => (
              <div key={key} className="argline">
                {node}
                {i < args.length - 1 ? <Cont /> : null}
              </div>
            ))}
          </>
        ) : (
          /* Single line: `❯ xkpasswd <arg> <arg>` */
          <div className="argline head">
            <span className="prompt">❯</span>{' '}
            <span className="bin">xkpasswd</span>
            {args.map(({ key, node }) => (
              <span key={key}> {node}</span>
            ))}
          </div>
        )}
      </div>

      {/* ── Command bar ──────────────────────────────────────────────────── */}
      <div className="cmdbar">
        <button onClick={onGenerate} title="regenerate">
          <RunIcon className="w-3.5 h-3.5" />
          run
        </button>
        <button
          onClick={handleCopy}
          className={copied ? 'done' : undefined}
          title="copy command to clipboard"
        >
          <CopyIcon className="w-3.5 h-3.5" />
          <span>{copied ? 'copied' : 'copy command'}</span>
        </button>
      </div>
    </div>
  );
};

export default ControlPanel;
