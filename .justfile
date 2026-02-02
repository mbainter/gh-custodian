@default:
  just --list

fmt:
  cargo fmt

clippy:
  cargo clippy --all-features --tests "$@"

test:
  cargo nextest run --no-fail-fast
