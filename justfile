
export RUSTFLAGS := "-D warnings"
export RUST_BACKTRACE := "1"

# yup
check-all:
    cargo nextest run --no-default-features
    cargo nextest run --no-default-features --features validation
    cargo nextest run --no-default-features --features location
    cargo nextest run --no-default-features --features dynamic
    cargo nextest run
    cargo test --doc

bump version:
    sed -Ei \
        -e 's/^version = ".*?"$/version = "{{version}}"/' \
        -e '/path = / s/version = ".*?"/version = "{{version}}"/' \
        Cargo.toml
    jj ci -m 'release: {{version}}'
    git rtag v{{version}}

publish: check-all
    cargo publish -p pinkie-parser
    cargo publish -p pinkie-macros
    cargo publish
