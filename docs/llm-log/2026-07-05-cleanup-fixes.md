# 2026-07-05 — Post-review cleanup fixes

Session: reviewed the project (written in an afternoon, untouched since) and
fixed the issues found, one by one.

## Fixes

| # | Issue | Fix |
|---|-------|-----|
| 1 | Dead deps | Removed `sha2` and `syn` from `pinkie-macros` (and `sha2` from workspace) |
| 2 | Column underflow panic in validation | `saturating_sub(8)` in `pinkie-macros/src/lib.rs`; also removed a commented-out debug block |
| 3 | Wrong MSRV (`1.60.0`) | Set `rust-version = "1.82.0"` (minimum for `Option::is_none_or`) |
| 4 | Naive brace matching in `dynamic` mode | Rewrote as `find_invocation` (identifier-boundary `css!` matching) + `find_block` (skips string/char/raw-string literals and nested block comments) in `src/lib.rs` |
| 5 | `dynamic` mode path assumptions undocumented | Documented on `collect()` and in readme; on failure it now falls back to the static embedded CSS instead of emitting an empty rule |
| 6 | Class hash truncated to 32 bits (birthday collisions) | Use full 16-hex-char xxh64; updated inline insta snapshots in `tests/collect.rs` |
| 7 | `PINKIE_CSS_CLASS_PREFIX` not tracked by cargo | Macro output now includes `const _: Option<&str> = option_env!("PINKIE_CSS_CLASS_PREFIX");` so rustc records the env dependency and rebuilds on change; dropped the `cargo clean` note from the readme |
| 8 | Stray `;` / `LINE` const indirection in macro output | Inlined `line!() as usize` into the struct literal, cleaned up the `quote!` block |
| 9 | Undocumented tokenization limits | Added a "Known limitations" readme section (CSS comments stripped, hex color tokenization, single quotes, string escapes, dynamic-mode path constraints) |
| 10 | Clippy nits | `f.write_str(self.class)` (needless ref), `replace(['\n', '\r'], " ")` in the parser |

Left alone: ugly-but-valid unminified output (inherent to the
token-concatenation design; lightningcss minification handles it).

## Decisions made (by necauqua)

- MSRV: just set 1.82, no cargo-msrv verification.
- Hash length: full 16 hex chars over 8 or 12.

## Notes

- `cargo test --all-features` exercises the dynamic scanner (debug assertions
  + `dynamic` feature make `collect()` read test sources at runtime); output
  matched the static path.
- The hash-length change changes **all class names** — release as at least a
  minor version bump.
- Verified: tests, clippy (zero warnings), `--no-default-features` and
  `--features dynamic` checks.
