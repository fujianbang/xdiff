# xdiff

[![Rust](https://github.com/fujianbang/xdiff/actions/workflows/rust.yml/badge.svg)](https://github.com/fujianbang/xdiff/actions/workflows/rust.yml)

A tool to complicated API easily.

## Usage

Using test case
```bash
# case 1
cargo run -- run -p rust -c fixtures/test.yaml -e a=100 -e @b=2 -e %c=3 -e m=100
# case 2
cargo run -- run -p todo -c fixtures/test.yaml -e a=100 -e @b=2 -e %c=3 -e m=100
```