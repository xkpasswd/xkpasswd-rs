# AGENTS.md

Guidance for AI agents and contributors working in **xkpasswd-rs** — an XKCD-style
password generator written in Rust, shipped as a CLI and as a Preact/WASM web app.

## Project shape

A single Rust crate (`xkpasswd`, edition 2021) produces three things, plus a web app:

- **Library** (`src/lib.rs`, crate-types `rlib` + `cdylib`) — the generation engine.
- **CLI** (`src/main.rs`, binary `xkpasswd`) — `clap`-based, gated by the `cli` feature.
- **WASM bindings** (`src/wasm/`) — `wasm-bindgen`, gated by the `wasm` feature, built
  **once per language** (en, de, es, fr, pt).
- **Web app** (`www/`) — Preact + TypeScript + Vite, consuming the per-language WASM.

The Rust engine is the source of truth. The web app re-implements only the *UI-facing*
pieces (command-string building, token editing) in TypeScript and delegates actual
password generation to WASM.

## Repository layout

```
src/                       Rust library + CLI
  lib.rs                   crate root (re-exports prelude, settings, bit_flags, wasm)
  main.rs                  CLI entry (feature: cli)
  bit_flags.rs             WordTransform enum + BitFlags trait  ← transform bit contract
  prelude/                 Xkpasswd engine, Language, Preset, Entropy/GuessTime
  settings/                Settings + Builder/Randomizer; word transform application
  cli/                     clap args + TOML config
  wasm/                    wasm-bindgen wrappers (camelCase JS API)
  assets/                  dict_<lang>.txt word lists (bundled into the binary/wasm)
www/                       Preact/TS/Vite web app
  src/ControlPanel/        settings UI: word/length/separator/padding/transforms/presets
  src/PasswordBox/         generated password display + copy
  src/Entropy/             entropy metrics
  src/DropdownButton/      reusable portal dropdown (positioning/dismiss reference)
  src/contexts.tsx         app state (settings, password, language) + dispatchers
  src/wasm.ts              dynamic per-language WASM import
  xkpasswd/                GENERATED wasm bindings (gitignored; do not edit/commit)
  scripts/wasm-pack-build.sh   per-language wasm-pack invocation
raw_assets/                source frequency lists + raw_dict_converter.py
docs/                      README images (xkcd-936.png, xkpasswd-web.png)
Makefile                   Rust + wasm build/test/lint orchestration
.github/workflows/         CI (cargo, wasm_pack, deploy)
```

## Build, test, lint

### Rust / CLI / WASM (run from repo root, via Makefile)

```shell
make lint        # cargo fmt --check + cargo check + clippy -D warnings (all-features)
make test        # test-cli + test-wasm (also enforces the wasm size limit)
make test-cli    # cargo test --frozen --all-features
make test-wasm   # builds per-lang wasm, checks size, then wasm-pack test --headless --firefox
make build       # build-cli + build-wasm (release, per-language)
make             # all = clean lint test build
make language-assets   # regenerate src/assets/dict_*.txt from raw_assets/
```

- WASM artifacts must stay **≤ 100 KB gzipped per language** (`WASM_BUNDLE_SIZE_LIMIT`,
  enforced by `make test-wasm-size` and CI).
- `wasm-pack test` requires Firefox; `make test-wasm` builds into `pkg/`.

### Web app (run from `www/`)

The canonical **pre-commit verification gates** for any web change are these four,
run individually (not the combined `npm run lint`):

```shell
npm run lint:ts                                   # tsc (type-check)
npm run lint:es                                   # eslint --fix (AUTOFIXES — re-stage if it edits files)
npm test                                          # vitest run
VITE_GIT_SHA=$(git rev-parse --short HEAD) npx vite build   # production build
```

Notes for agents:

- Do **not** run bare `tsc` from an arbitrary directory — use `npm run lint:ts` (runs in `www/`).
- `lint:es` runs `eslint --fix`, so it can modify files; check `git status` and re-stage.
- `npm run dev` / `npm run build` / `npm run serve` trigger `predev`/`prebuild` hooks that
  rebuild all 5 WASM modules (slow). When the `www/xkpasswd/` bindings already exist and
  you only need to verify TS/build output, call **`npx vite build`** directly to skip the
  wasm rebuild (as the gates above do). Use `npx vite preview` to serve the existing `dist/`.
- Toolchain: Preact 10, Vite 5, Vitest 2, TypeScript 5 (strict), ESLint 8 (+ prettier), Tailwind 3.
- Dev server defaults to Vite's `http://localhost:5173`; preview to `:4173`.

## WASM per-language build

`www/scripts/wasm-pack-build.sh <dev|prod> <lang>` runs:

```shell
wasm-pack build <profile> --target=web \
  --out-dir=www/xkpasswd --out-name=xkpasswd-<lang> \
  --no-default-features --features=<wasm|wasm_dev> --features=lang_<lang>
```

with `RUSTFLAGS="-C target-feature=+bulk-memory"`. Output lands in `www/xkpasswd/`
(`xkpasswd-<lang>.js` + `xkpasswd-<lang>_bg.wasm`), which is generated and gitignored.

## Word transforms — shared bit-flag contract

Transforms are bit flags, defined in `src/bit_flags.rs` and mirrored in the web layer
(`www/src/ControlPanel/`):

| Bit | Name                 | CLI name (`--transforms`)  |
|-----|----------------------|----------------------------|
| 1   | Lowercase            | `lowercase`                |
| 2   | Titlecase            | `titlecase`                |
| 4   | Uppercase            | `uppercase`                |
| 8   | InversedTitlecase    | `inversed-titlecase`       |
| 64  | AltercaseLowerFirst  | `altercase-lower-first`    |
| 128 | AltercaseUpperFirst  | `altercase-upper-first`    |

Legal-combination rules (enforced in Rust `settings`, mirrored by the web canonicalizer):

- At least one transform must be set.
- The two **case** groups: simple cases (1/2/4/8) may be combined (engine picks one at
  random per word); **altercase** (64/128) is mutually exclusive and **replaces** all
  simple cases. Never emit altercase *and* a simple case together.

Web specifics (added in `feat(www): restore altercase parity via word-transforms popover`):

- `www/src/ControlPanel/editing.ts` holds the pure helpers: `activeAltercase`,
  `selectedCases`, `toggleCase`, `toggleAltercase`, and `canonicalTransforms`.
- The web state (`wordTransforms` in `contexts.tsx`) may store simple-case bits **and** an
  altercase bit together so the prior case selection survives a popover close/reopen.
  All output boundaries (token display, copied command via
  `www/src/ControlPanel/cmdString.ts`, and the engine) derive a **canonical** projection:
  in altercase mode exactly one `--transforms=altercase-*` flag is shown.
- The transforms UI is a shared positioned popover (`WordTransformsMenu.tsx`): 4 case
  checkboxes + 2 mutually-exclusive altercase radios; the last remaining case locks.

## Conventions

- **Commits**: `type(scope): subject` (e.g. `feat(www): …`, `fix(ui): …`, `chore(cli): …`).
  One logical change per commit (surgical commits). Keep history **linear — rebase, don't
  merge**. Commits are **GPG-signed automatically** (`commit.gpgsign=true`); do not pass
  `-S` yourself, and note the displayed SHA may differ from a pre-sign value.
- **Scope of web work**: the web app must not require Rust/WASM changes for UI-only work;
  keep the TS/Rust transform contract in sync if you touch bit semantics on either side.
- **License**: GPLv3. **Node**: 24 in CI. **Rust**: stable, edition 2021.

## Gotchas

- Vite "Could not resolve `../xkpasswd/…`" after re-running wasm-pack → clear the cache:
  `rm -rf www/node_modules/.vite`.
- `npm run lint:es` autofixes; commits can drift if you forget to re-stage.
- Generated `www/xkpasswd/` and `www/dist/` are gitignored — never commit them.

## CI (.github/workflows/)

- `cargo.yml` — fmt, check, clippy, test, coverage (Rust PR gate).
- `wasm_pack.yml` — wasm-pack tests (Chrome/Firefox) + per-language gzip size assertion.
- `production_deployment.yaml` / `staging_deployment.yaml` — build `www/dist/` (Node 24)
  and deploy to GitHub Pages (`https://xkpasswd.github.io`).
