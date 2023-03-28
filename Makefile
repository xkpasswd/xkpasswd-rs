.PHONY: all clean lint test test-cli test-wasm build build-cli build-wasm languages en fr pt
LANGUAGES = en de es fr pt
CARGO_TEST_PARAMS = --frozen --all-features
CARGO_RELEASE_PARAMS = --frozen --release --no-default-features

all: clean lint test build

clean:
	@cargo clean; \
		rm -rf pkg

lint:
	@cargo fmt --all -- --check; \
	cargo check $(CARGO_TEST_PARAMS); \
	cargo clippy $(CARGO_TEST_PARAMS) -- -D warnings

test: test-cli test-wasm

test-cli:
	@cargo test $(CARGO_TEST_PARAMS)

test-wasm: test-wasm-size
	@wasm-pack test --headless --firefox $(CARGO_TEST_PARAMS)

test-wasm-size: build-wasm $(addprefix test-wasm-size-, $(LANGUAGES))

$(addprefix build-wasm-, $(LANGUAGES)):
	@lang=$(@:build-wasm-%=%); \
	wasm-pack build --out-name=xkpasswd-"$$lang" $(CARGO_RELEASE_PARAMS) --features=wasm --features=lang_"$$lang"

$(addprefix test-wasm-size-, $(LANGUAGES)):
	@lang=$(@:test-wasm-size-%=%); \
	bundle_size=$$(gzip -9 < pkg/xkpasswd-"$$lang"_bg.wasm | wc -c); \
	size_limit=$$([ "$$lang" = en ] && echo 100000 || echo 150000); \
	printf "wasm build bundle size for '%s': " "$$lang"; \
	[ "$$bundle_size" -gt "$$size_limit" ] \
		&& echo "bundle size exceeds limit ($$bundle_size > $$size_limit bytes)" && exit 1 \
		|| echo "bundle size ok"

build: build-cli build-wasm

build-cli:
	@cargo build $(CARGO_RELEASE_PARAMS) --features=cli --features=all_langs

build-wasm: $(addprefix build-wasm-, $(LANGUAGES))

language-assets:
	@cd raw_assets; \
		./raw_dict_converter.py; \
		mv dict_*.txt "../src/assets"
