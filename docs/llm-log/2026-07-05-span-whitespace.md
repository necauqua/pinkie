# 2026-07-05 — Span-based whitespace handling

Implemented the "possible future work" item from
[2026-07-05-test-improvements.md](2026-07-05-test-improvements.md): the parser
now uses token span locations to decide whether tokens are joined or
separated by a single space, instead of guessing from token kinds.

## What changed

- **`pinkie-parser/src/lib.rs`** — rewrote `parse_recursive`: for each token,
  compare `span().start()` against the previous token's `span().end()`.
  Adjacent → no space; any gap (including newlines) → a single space. All the
  old heuristics (`-`/`:`/`&` punct joining, kebab-case lookahead, `%` suffix
  special case, hardcoded spaces inside group delimiters) are gone.
  `Delimiter::None` groups now recurse transparently without emitting
  anything. Raw strings still go through `syn::LitStr` with newlines replaced
  by spaces (keeps the error-span offset mapping single-line).
- **`Cargo.toml`** — `proc-macro2` pinned to `>=1.0.95` with the
  `span-locations` feature (1.0.95+ resolves real span locations on stable
  when compiled with Rust 1.88+); **MSRV bumped 1.82.0 → 1.88.0**.
- **`readme.md`** — reworded the tokenization blurb (whitespace is now
  reproduced exactly), removed the two fixed quirks from known limitations.

## Quirks fixed

- `calc(100% - 2px)` — emitted verbatim (previously `calc( 100% -2px )`,
  silently invalid in the browser).
- `input[type="text"]` — stays an attribute selector (previously the added
  space made it a descendant selector). Grid line names (`auto [col]`) keep
  their space too — new `grid_line_names` test proves both directions.
- General output cleanup: `color: red;` instead of `color:red ;`,
  `!important` instead of `! important`, etc.

Still unfixable: `1.5em` (Rust float-exponent lexing) — remains in readme
known limitations with the raw-string workaround.

## Notes

- The dynamic mode runtime path uses proc-macro2 *fallback* spans (parsing
  source substrings at runtime) — with `span-locations` those are relative to
  the parsed string, so adjacency comparison is consistent there too.
- **All class hashes changed** (whitespace is part of the hashed CSS string) —
  release as at least a minor version bump.
- Updated inline insta snapshots in `tests/whitespaces.rs` and
  `tests/collect.rs`; the two "documents a known quirk" comments now describe
  the correct behavior.

## Also

- Added `AGENTS.md` at the project root: workspace layout, key invariants
  (hash-derived class names, static/dynamic parser output parity, the `1.5em`
  quirk), and `just check-all` as the canonical verification command.

## Verified

- `cargo test` under `--all-features`, `--no-default-features`, and
  `--no-default-features --features dynamic` — all green (including the
  dynamic scanner end-to-end test).
- clippy `--all-features --all-targets` — zero warnings.
- `just check-all` is the canonical verification command; it was missing two
  combos, now added: `cargo nextest run --all-features` (validation + dynamic
  together) and the clippy run above (both under `RUSTFLAGS="-D warnings"`).
- Validation error spans still point at the offending token (checked with a
  scratch crate: `colr ~~ nonsense;` underlines the `~`).
