.PHONY: all clean lint languages test test-cli test-wasm build build-cli build-wasm
LANGUAGES = en fr pt

all: clean lint test build

clean:
	@cargo clean; \
		rm -rf pkg; \
		rm -rf www/{dist,xkpasswd}

lint:
	@cargo fmt --all -- --check; \
		cargo check --features=cli_dev --features=wasm_dev; \
		cargo clippy --features=cli_dev --features=wasm_dev -- -D warnings

test: test-cli test-wasm

test-cli:
	@cargo test --features=cli_dev

test-wasm:
	@wasm-pack test --headless --firefox --features=wasm_dev

build: build-cli build-wasm

build-cli:
	@cargo build --release --no-default-features --features=cli

build-wasm:
	@wasm-pack build --no-default-features --features=wasm

languages: $(LANGUAGES)

$(LANGUAGES): 
	@cd raw_assets; \
	DICT=$@ ./convert_dict.py; \
	mv "dict_$@.txt" "../src/assets"
