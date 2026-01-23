.PHONY: all clean lint test test-cli test-wasm test-wasm-browser test-wasm-size build build-cli build-wasm language-assets check-wasm-tools

CARGO_TEST_PARAMS = --frozen --all-features
CARGO_RELEASE_PARAMS = --frozen --release --no-default-features
WASM_TARGET = wasm32-unknown-unknown

# Supported languages
LANGUAGES = en de es fr pt

# Bundle size limit for Wasm files is 100KB (gzipped)
WASM_BUNDLE_SIZE_LIMIT = 100000

# wasm-opt flags for modern browser features (optional optimization)
WASM_OPT_FLAGS = -O --enable-bulk-memory --enable-nontrapping-float-to-int

all: clean lint test build

clean:
	@cargo clean; \
		rm -rf pkg

lint:
	@cargo fmt --all -- --check; \
	cargo check $(CARGO_TEST_PARAMS); \
	cargo clippy $(CARGO_TEST_PARAMS) -- -D warnings

# Check that required WASM tools are installed
check-wasm-tools:
	@command -v wasm-bindgen >/dev/null 2>&1 || { \
		echo "wasm-bindgen-cli not found. Install with: cargo install wasm-bindgen-cli"; \
		exit 1; \
	}

test: test-cli test-wasm

test-cli:
	@cargo test $(CARGO_TEST_PARAMS)

# WASM tests - builds and verifies size constraints
# For browser-based tests, use: make test-wasm-browser
test-wasm: test-wasm-size
	@echo "WASM tests passed (build + size verification)"

# Browser-based WASM tests using wasm-bindgen-test-runner
# Requires: geckodriver (Firefox) or chromedriver (Chrome) in PATH
test-wasm-browser: check-wasm-tools
	@echo "Running WASM browser tests in Firefox..."
	@CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
		WASM_BINDGEN_TEST_TIMEOUT=60 \
		cargo test --target $(WASM_TARGET) $(CARGO_TEST_PARAMS)

test-wasm-size: build-wasm $(addprefix test-wasm-size-, $(LANGUAGES))

$(addprefix test-wasm-size-, $(LANGUAGES)):
	@lang=$(@:test-wasm-size-%=%); \
	bundle_size=$$(gzip -9 < pkg/xkpasswd-"$$lang"_bg.wasm | wc -c); \
	printf "wasm build bundle size for '%s': " "$$lang"; \
	[ "$$bundle_size" -gt "$(WASM_BUNDLE_SIZE_LIMIT)" ] \
		&& echo "bundle size exceeds limit ($$bundle_size > $(WASM_BUNDLE_SIZE_LIMIT) bytes)" && exit 1 \
		|| echo "bundle size ok"

build: build-cli build-wasm

build-cli:
	@cargo build $(CARGO_RELEASE_PARAMS) --features=cli --features=all_langs

build-wasm: check-wasm-tools $(addprefix build-wasm-, $(LANGUAGES))
	@echo "WASM builds complete in pkg/"

# Build WASM for each language using cargo + wasm-bindgen + wasm-opt (optional)
$(addprefix build-wasm-, $(LANGUAGES)):
	@lang=$(@:build-wasm-%=%); \
	echo "Building WASM for language: $$lang"; \
	mkdir -p pkg; \
	cargo build --lib --target $(WASM_TARGET) $(CARGO_RELEASE_PARAMS) --features=wasm --features=lang_"$$lang"; \
	wasm-bindgen target/$(WASM_TARGET)/release/xkpasswd.wasm \
		--out-dir pkg \
		--out-name xkpasswd-"$$lang" \
		--target web; \
	if command -v wasm-opt >/dev/null 2>&1; then \
		wasm-opt pkg/xkpasswd-"$$lang"_bg.wasm $(WASM_OPT_FLAGS) -o pkg/xkpasswd-"$$lang"_bg.wasm; \
		echo "Built and optimized pkg/xkpasswd-$$lang.js"; \
	else \
		echo "Built pkg/xkpasswd-$$lang.js (wasm-opt not found, skipping optimization)"; \
	fi

language-assets:
	@cd raw_assets; \
	./raw_dict_converter.py; \
	mv dict_*.txt "../src/assets"

# Development helpers
.PHONY: install-wasm-tools
install-wasm-tools:
	@echo "Installing WASM toolchain..."
	rustup target add $(WASM_TARGET)
	cargo install wasm-bindgen-cli
	@echo ""
	@echo "For wasm-opt (optional, for size optimization):"
	@echo "  macOS: brew install binaryen"
	@echo "  Linux: apt install binaryen"
	@echo "  Or download from: https://github.com/WebAssembly/binaryen/releases"
	@echo ""
	@echo "For browser tests (make test-wasm-browser):"
	@echo "  Firefox: brew install geckodriver"
	@echo "  Chrome: brew install chromedriver"
