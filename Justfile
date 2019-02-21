ci: test build fmt clippy doc

test:
  cargo test --all --color=always --features=ci -- --test-threads=1 --quiet

build:
  cargo build --features=ci

fmt:
  cargo fmt -- --check

clippy:
  cargo clippy --color=always -- --deny clippy::all

doc:
  cargo doc

dev:
  cargo test --all --color=always --features=dev -- --test-threads=1 --quiet
  ag fixme

run_bigger:
  cargo run -- tests/examples/bigger/script

test_dockerfile:
  docker build -t check-protocols .
  docker run --rm \
    --cap-add=SYS_PTRACE \
    -v $(pwd)/tests/examples/bigger/script:/root/script \
    -v $(pwd)/tests/examples/bigger/script.protocol.yaml:/root/script.protocol.yaml \
    check-protocols \
    script