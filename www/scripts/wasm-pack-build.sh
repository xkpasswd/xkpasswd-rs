#!/usr/bin/env bash

function main() {
	local env=${1:-}
	local lang=${2:-}

	local wasm_feature="wasm_dev"
	local profile="--dev"

	if [[ "$env" = "prod" ]]; then
		wasm_feature="wasm"
		profile="--release"
	fi

	# Set RUSTFLAGS to enable bulk-memory for wasm32 target
	# This is required for newer Rust versions that generate bulk memory operations
	export RUSTFLAGS="-C target-feature=+bulk-memory"

	wasm-pack build \
		$profile \
		--target=web \
		--out-dir=www/xkpasswd \
		--out-name="xkpasswd-$lang" \
		--no-default-features \
		--features="$wasm_feature" \
		--features="lang_$lang"
}

main "$@"
