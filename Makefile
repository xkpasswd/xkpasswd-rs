.PHONY: all clean lint test test-cli test-wasm build build-cli build-wasm languages en fr pt

all: clean lint test build

clean:
	@cargo clean; \
		rm -rf pkg

lint:
	@cargo fmt --all -- --check; \
		cargo check --features=cli_dev --features=wasm_dev; \
		cargo clippy --features=cli_dev --features=wasm_dev -- -D warnings

test: test-cli test-wasm

test-cli:
	@cargo test --features=cli_dev

test-wasm:
	@wasm-pack test --headless --firefox --features=wasm_dev

build: build-cli

build-cli:
	@cargo build --release --no-default-features --features=cli --features=all_langs

languages:
	@cd raw_assets; \
		./raw_dict_converter.py; \
		mv dict_*.txt "../src/assets"
