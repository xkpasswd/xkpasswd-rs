.PHONY: all clean lint test test-cli test-wasm build build-cli build-wasm languages en fr pt
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
	@cargo build --release --no-default-features --features=cli --features=all_langs

build-wasm: build-wasm-en build-wasm-fr build-wasm-pt

build-wasm-en:
	@wasm-pack build --out-name=xkpasswd-en --no-default-features --features=wasm --features=lang_en

build-wasm-fr:
	@wasm-pack build --out-name=xkpasswd-fr --no-default-features --features=wasm --features=lang_fr

build-wasm-pt:
	@wasm-pack build --out-name=xkpasswd-pt --no-default-features --features=wasm --features=lang_pt

languages: $(LANGUAGES)

$(LANGUAGES): 
	@cd raw_assets; \
	DICT=$@ ./convert_dict.py; \
	mv "dict_$@.txt" "../src/assets"
