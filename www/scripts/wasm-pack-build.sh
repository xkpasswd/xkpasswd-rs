#!/usr/bin/env bash

function main() {
  local env=${1:-}
  local lang=${2:-}

  local wasm_feature="wasm_dev"

  if [[ "$env" = "prod" ]]; then
    wasm_feature="wasm"
  fi

  wasm-pack build \
    --target=web \
    --out-dir=www/xkpasswd \
    --out-name="xkpasswd-$lang" \
    --no-default-features \
    --features="$wasm_feature" \
    --features="lang_$lang"
}

main "$@"
