.PHONY: lint test

all: lint test

lint:
	@cargo fmt; \
		cargo check --features=cli_dev --features=wasm_dev; \
		cargo clippy --features=cli_dev --features=wasm_dev

test: test-cli test-wasm

test-cli:
	@cargo test --features=cli_dev

test-wasm:
	@wasm-pack test --headless --firefox --features=wasm_dev
