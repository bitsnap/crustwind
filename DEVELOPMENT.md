## Development

Install [asdf](https://asdf-vm.com/) first.

```bash
asdf plugin add rust
asdf install
asdf current # should list all the correct versions from .tools-versions file

rustup default nightly
rustup component add clippy-preview
rustup component add rustfmt
rustup component add miri

cargo install cargo-nextest --locked
cargo install cargo-mutants --locked
cargo install cargo-expand --locked
cargo install cargo-sonar --locked
cargo install cargo-llvm-cov --locked
cargo install licensure --locked

# double check that rust sources are available and change `Cargo.toml` path, if necessary
rust_nightly_date="2025-05-13" # rustc 1.89.0
rustup install "nightly-${rust_nightly_date}" 

cargo build

# Testing
cargo nextest r --all  
cargo mutants --workspace -j 32    # for mutation testing with 32 concurrent jobs
cargo miri nextest run --all -j 32 # for miri mem-leak testing
cargo miri test
cargo miri run

# Coverage reporting
cargo llvm-cov --workspace --html --open

# Linting & Formatting
licensure -i crustwind/**/src/**/*.rs
rustfmt --edition 2024 crustwind/**/src/**/*.rs

# commit changes
cargo clippy --fix  
```
