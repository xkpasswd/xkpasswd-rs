# xkpasswd-rs Modernization Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Upgrade xkpasswd-rs to compile and run stably with Rust 1.93+, updating all dependencies, refactoring to 2026 standards, and modernizing CI/CD.

**Architecture:** Phased approach starting with critical compilation fixes, then dependency upgrades, code modernization, CI/CD updates, and finally frontend updates. Each phase is independently testable.

**Tech Stack:** Rust 1.93, wasm-pack, wasm-bindgen 0.2.108, Vite 6.x, Preact 10.x

---

## Phase 1: Critical Compilation Fixes

### Task 1.1: Update wasm-bindgen ecosystem

**Files:**
- Modify: `Cargo.toml:48-52`
- Modify: `Cargo.lock` (auto-generated)

**Step 1: Update Cargo.toml wasm dependencies**

```toml
wasm-bindgen = "0.2.108"
web-sys = { version = "0.3.80", features = ["console"], optional = true }

[dev-dependencies]
wasm-bindgen-test = "0.3.45"
```

**Step 2: Run cargo update**

```bash
cargo update -p wasm-bindgen -p web-sys -p wasm-bindgen-test
```

**Step 3: Verify compilation**

```bash
cargo check --all-features
```
Expected: Success (no compilation errors)

**Step 4: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "fix(01-01): update wasm-bindgen ecosystem to 0.2.108"
```

---

### Task 1.2: Run and fix clippy warnings

**Files:**
- Potentially modify: `src/prelude/tests.rs`

**Step 1: Run clippy**

```bash
cargo clippy --all-features -- -D warnings 2>&1
```

**Step 2: Fix any warnings found**

Common issues to expect:
- `assert_eq!(false, ...)` -> `assert!(!...)`
- Unused variables
- Redundant clones

**Step 3: Verify clippy passes**

```bash
cargo clippy --all-features -- -D warnings
```
Expected: No warnings

**Step 4: Commit**

```bash
git add -A
git commit -m "fix(01-02): resolve clippy warnings for Rust 1.93"
```

---

## Phase 2: Dependency Upgrades

### Task 2.1: Update toml crate (breaking change)

**Files:**
- Modify: `Cargo.toml:47`
- Modify: `src/cli/toml_conf.rs:124-155`

**Step 1: Update Cargo.toml**

```toml
toml = { version = "0.8", optional = true }
```

**Step 2: Fix breaking API change in toml_conf.rs**

Replace `toml::from_slice()` with `toml::from_str()`. Change `fs::read()` to `fs::read_to_string()`:

```rust
fn read_config_file(config_file: &Option<String>) -> Result<toml::Value, ConfigParseError> {
    let file_data = match config_file {
        Some(config_file) => match fs::read_to_string(config_file) {
            Ok(data) => {
                log::debug!("found config file at custom path {}", config_file);
                Ok(data)
            }
            Err(err) => Err(ConfigParseError::InvalidFile(err.to_string())),
        },
        None => match lookup_default_config_path() {
            None => {
                log::debug!("config file at default path not found, ignoring");
                Err(ConfigParseError::Ignore)
            }
            Some(config_path) => match fs::read_to_string(&config_path) {
                Ok(data) => {
                    log::debug!("found config file at default path {}", config_path);
                    Ok(data)
                }
                Err(err) => Err(ConfigParseError::InvalidFile(err.to_string())),
            },
        },
    };

    match file_data {
        Err(err) => Err(err),
        Ok(data) => match toml::from_str::<toml::Value>(&data) {
            Err(parse_err) => Err(ConfigParseError::InvalidFile(parse_err.to_string())),
            Ok(parsed_data) => Ok(parsed_data),
        },
    }
}
```

**Step 3: Run tests**

```bash
cargo test --all-features
```
Expected: All tests pass

**Step 4: Commit**

```bash
git add Cargo.toml Cargo.lock src/cli/toml_conf.rs
git commit -m "feat(02-01): upgrade toml crate to 0.8 with API migration"
```

---

### Task 2.2: Update clap to latest 4.x

**Files:**
- Modify: `Cargo.toml:40`

**Step 1: Update Cargo.toml**

```toml
clap = { version = "4.5", features = ["derive"], optional = true }
```

**Step 2: Update dependencies**

```bash
cargo update -p clap
```

**Step 3: Run tests**

```bash
cargo test --all-features
```
Expected: All tests pass (clap 4.5 is backward compatible)

**Step 4: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "feat(02-02): upgrade clap to 4.5"
```

---

### Task 2.3: Update remaining dependencies

**Files:**
- Modify: `Cargo.toml:42,44,46`

**Step 1: Update Cargo.toml**

```toml
dirs = { version = "5.0", optional = true }
getrandom = { version = "0.2.15", features = ["js"], optional = true }
log = { version = "0.4.22", optional = true }
stderrlog = { version = "0.6", optional = true }
```

**Step 2: Update dependencies**

```bash
cargo update
```

**Step 3: Run tests**

```bash
cargo test --all-features
```
Expected: All tests pass

**Step 4: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "feat(02-03): upgrade dirs, getrandom, log, stderrlog"
```

---

## Phase 3: Code Modernization (2026 Standards)

### Task 3.1: Replace assert_eq!(false, ...) with assert!(!)

**Files:**
- Modify: `src/prelude/tests.rs:89,109,129,149,169`

**Step 1: Fix boolean assertions**

Replace all instances of:
```rust
assert_eq!(false, pass.dict.is_empty());
```
With:
```rust
assert!(!pass.dict.is_empty());
```

**Step 2: Run tests**

```bash
cargo test --all-features
```

**Step 3: Commit**

```bash
git add src/prelude/tests.rs
git commit -m "refactor(03-01): use idiomatic boolean assertions"
```

---

### Task 3.2: Improve error handling with graceful fallbacks

**Files:**
- Modify: `src/prelude/mod.rs:291-300`

**Step 1: Improve dict loading error handling**

Replace `.unwrap()` with graceful handling:

```rust
dict_str.lines().for_each(|line| {
    let mut comps = line.trim().split(':');

    if let Some(len_str) = comps.next() {
        let len = match len_str.parse::<u8>() {
            Ok(l) => l,
            Err(_) => {
                log::warn!("invalid word length in dictionary: {}", len_str);
                return;
            }
        };
        let words_csv = comps.next().unwrap_or("");
        let words: Vec<&str> = words_csv.split(',').collect();
        dict.insert(len, words);
    }
});
```

**Step 2: Run tests**

```bash
cargo test --all-features
```

**Step 3: Commit**

```bash
git add src/prelude/mod.rs
git commit -m "refactor(03-02): improve error handling in dict loader"
```

---

### Task 3.3: Add crate-level documentation

**Files:**
- Modify: `src/lib.rs:1`

**Step 1: Add crate documentation**

Add at the top of lib.rs:
```rust
//! # xkpasswd
//!
//! XKCD-style password generator written in Rust with WASM support.
//!
//! Inspired by [XKCD #936](https://xkcd.com/936/).
//!
//! ## Features
//!
//! - CLI application for generating passwords
//! - WASM module for web integration
//! - Multiple language support (EN, DE, ES, FR, PT)
//! - Configurable presets (AppleID, Web16, Web32, WiFi, etc.)
```

**Step 2: Verify docs build**

```bash
cargo doc --all-features --no-deps
```

**Step 3: Commit**

```bash
git add src/lib.rs
git commit -m "docs(03-03): add crate-level documentation"
```

---

## Phase 4: CI/CD Modernization

### Task 4.1: Update cargo.yml workflow

**Files:**
- Modify: `.github/workflows/cargo.yml`

**Step 1: Replace deprecated actions**

Replace entire file with:
```yaml
on: [pull_request]

name: Cargo

jobs:
  check:
    name: Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo check --all-features

  fmt:
    name: Fmt
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --all -- --check

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - run: cargo clippy --all-features -- -D warnings

  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
```

**Step 2: Commit**

```bash
git add .github/workflows/cargo.yml
git commit -m "ci(04-01): migrate to dtolnay/rust-toolchain"
```

---

### Task 4.2: Update wasm_pack.yml workflow

**Files:**
- Modify: `.github/workflows/wasm_pack.yml`

**Step 1: Update workflow**

Replace entire file with:
```yaml
on: [pull_request]

name: wasm-pack

jobs:
  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      - name: Test on Chrome
        run: wasm-pack test --headless --chrome --all-features
      - name: Test on Firefox
        run: wasm-pack test --headless --firefox --all-features

  build-size:
    name: Build size limit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      - name: Assert bundle size (gzipped)
        run: make test-wasm-size
```

**Step 2: Commit**

```bash
git add .github/workflows/wasm_pack.yml
git commit -m "ci(04-02): update wasm workflow with modern toolchain"
```

---

### Task 4.3: Update deployment workflows

**Files:**
- Modify: `.github/workflows/production_deployment.yaml`
- Modify: `.github/workflows/staging_deployment.yaml`

**Step 1: Update production deployment**

Replace entire file with:
```yaml
name: Production deployment

on:
  push:
    branches:
      - main

jobs:
  deploy:
    name: Deploy
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4
        with:
          persist-credentials: false
          lfs: true

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 22

      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      - name: Install and Build
        run: |
          cd www
          npm install
          npm run build

      - name: Deploy to GitHub Pages
        uses: JamesIves/github-pages-deploy-action@v4
        with:
          token: ${{ secrets.GHPAGES_TOKEN }}
          repository-name: xkpasswd/xkpasswd.github.io
          branch: gh-pages
          folder: www/dist
          clean: true
```

**Step 2: Apply similar changes to staging workflow**

**Step 3: Commit**

```bash
git add .github/workflows/production_deployment.yaml .github/workflows/staging_deployment.yaml
git commit -m "ci(04-03): modernize deployment workflows"
```

---

## Phase 5: Frontend Updates

### Task 5.1: Update npm dependencies

**Files:**
- Modify: `www/package.json`
- Modify: `www/package-lock.json` (auto-generated)

**Step 1: Update package.json dependencies**

```json
{
  "dependencies": {
    "@heroicons/react": "^2.2.0",
    "preact": "^10.25.0"
  },
  "devDependencies": {
    "@preact/preset-vite": "^2.10.0",
    "@typescript-eslint/parser": "^8.0.0",
    "autoprefixer": "^10.4.20",
    "concurrently": "^9.0.0",
    "eslint": "^8.57.0",
    "eslint-config-preact": "^1.5.0",
    "eslint-config-prettier": "^9.1.0",
    "postcss": "^8.5.0",
    "prettier": "^3.4.0",
    "tailwindcss": "^3.4.0",
    "typescript": "^5.7.0",
    "vite": "^6.0.0",
    "vite-plugin-top-level-await": "^1.5.0",
    "vite-plugin-wasm": "^3.4.0",
    "vite-tsconfig-paths": "^5.1.0"
  }
}
```

Note: Keeping ESLint at 8.x to avoid flat config migration complexity.

**Step 2: Delete package-lock.json and reinstall**

```bash
cd www
rm package-lock.json
npm install
```

**Step 3: Test build**

```bash
npm run build
```

**Step 4: Commit**

```bash
git add www/package.json www/package-lock.json
git commit -m "feat(05-01): upgrade npm dependencies to 2026 versions"
```

---

### Task 5.2: Fix any TypeScript/build issues from upgrades

**Files:**
- Potentially modify: `www/tsconfig.json`, `www/vite.config.ts`

**Step 1: Run lint check**

```bash
cd www && npm run lint
```

**Step 2: Fix any TypeScript or build issues**

**Step 3: Verify build**

```bash
npm run build
```

**Step 4: Commit if changes needed**

```bash
git add -A
git commit -m "fix(05-02): resolve build issues after dependency upgrade"
```

---

## Phase 6: Final Verification

### Task 6.1: Full test suite

**Step 1: Run all Rust tests**

```bash
cargo test --all-features
```

**Step 2: Run clippy**

```bash
cargo clippy --all-features -- -D warnings
```

**Step 3: Run fmt check**

```bash
cargo fmt --all -- --check
```

**Step 4: Build CLI**

```bash
cargo build --release --no-default-features --features=cli --features=all_langs
```

**Step 5: Test CLI works**

```bash
./target/release/xkpasswd --help
./target/release/xkpasswd -P xkcd
```

**Step 6: Build WASM**

```bash
make build-wasm
```

**Step 7: Build web frontend**

```bash
cd www && npm run build
```

**Step 8: Commit any final fixes**

---

### Task 6.2: Update README if needed

**Files:**
- Potentially modify: `README.md`

**Step 1: Verify all build instructions still work**

**Step 2: Update any outdated information (Node version, etc.)**

**Step 3: Commit if changes made**

```bash
git add README.md
git commit -m "docs(06-02): update README for modernized project"
```

---

## Summary

| Phase | Tasks | Description |
|-------|-------|-------------|
| 1 | 2 | Critical compilation fixes |
| 2 | 3 | Dependency upgrades |
| 3 | 3 | Code modernization |
| 4 | 3 | CI/CD updates |
| 5 | 2 | Frontend updates |
| 6 | 2 | Final verification |

**Total Tasks:** 15 tasks across 6 phases
**Estimated Time:** 2-3 hours for full implementation
**Risk Level:** Medium (toml migration is main breaking change)

**Execution Order:**
1. Phase 1 must complete first (unblocks compilation)
2. Phases 2-4 can be done in parallel or any order
3. Phase 5 depends on Phase 1 (needs WASM build working)
4. Phase 6 is final verification
