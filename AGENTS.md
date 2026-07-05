# Agent notes for pinkie

Pinkie is a scoped CSS-in-Rust crate: a `css!` proc macro turns Rust tokens
into a CSS string under a content-hashed class, collected at runtime via
`inventory`. Read `readme.md` first — it's the crate-level doc
(`#![doc = include_str!]`) and its examples are doctests.

## Workspace layout

- `src/lib.rs` — the `pinkie` crate: `Style`, `collect()`, and the
  `dynamic`-mode source scanner (`find_invocation`/`find_block`, which skip
  strings/comments when brace-matching).
- `pinkie-macros/` — the `css!` proc macro: parses via `pinkie-parser`,
  optionally validates with `lightningcss` (mapping error locations back to
  token spans), hashes with xxh64, emits a `Style` + `inventory::submit`.
- `pinkie-parser/` — shared token→CSS conversion, used both by the macro at
  compile time and by `dynamic` mode at runtime. Whitespace is decided from
  **token span locations** (adjacent = joined, any gap = one space); requires
  proc-macro2 ≥1.0.95 with `span-locations` and Rust ≥1.88 — that's what
  pins the MSRV.

## Key invariants

- The class name is a hash of the generated CSS string. **Any change to the
  parser output changes every class name** — expect insta snapshot churn in
  tests, and point it out to the user (it affects versioning).
- `pinkie-parser` output must be identical whether tokens come from the real
  compiler (proc macro) or from proc-macro2's fallback parser (`dynamic` mode
  re-reading source files at runtime) — `tests/dynamic.rs` asserts this
  end-to-end and panics on the fallback warning so degradation can't hide.
- `dynamic` mode only works when CWD is the workspace root (`file!()` paths);
  on any failure it logs a warning and falls back to the static CSS.
- Known unfixable tokenization quirk: `1.5em` (Rust float-exponent lexing).
  Documented in readme "Known limitations"; `r"raw strings"` are the escape
  hatch. Don't try to fix it in the parser.

## Verification

- `just check-all` is the canonical check: nextest across all feature combos
  (incl. `--all-features` = validation+dynamic), doc tests, and clippy, all
  under `RUSTFLAGS="-D warnings"`. Run it before declaring work done.
- Tests use inline insta snapshots; `cargo insta test --accept --all-features`
  to update after intentional output changes.
- Feature flags: `validation` and `location` are default; `dynamic` implies
  `location` and only does anything with debug assertions on.
