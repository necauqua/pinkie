# 2026-07-05 — Test suite improvements

Follow-up to the cleanup session ([2026-07-05-cleanup-fixes.md](2026-07-05-cleanup-fixes.md)):
expanded the test suite and documented the quirks it surfaced.

## What was added

- **`tests/whitespaces.rs`** — 8 new parser cases: hex colors, negative
  values, time units, strings, raw-string passthrough, `!important`, plus two
  that document quirks (see below).
- **`tests/dedup.rs`** (new) — identical `css!` bodies produce the same class
  and only one rule in `collect()`; class name format (`pinkie-` + 16 hex
  chars).
- **`tests/dynamic.rs`** (new) — end-to-end dynamic mode: a `css!` block
  stuffed with scanner traps (braces in line/nested-block comments,
  `content: "}"`, raw strings), plus a `Log` impl that panics on the fallback
  warning — so a silently-degraded dynamic path fails the test instead of
  passing via fallback.
- **`src/lib.rs`** — 5 unit tests for `find_invocation`/`find_block` covering
  each skip case in isolation: identifier boundaries (`my_css!` doesn't
  match), nested comments, escaped strings, raw strings with mismatched-hash
  closers, char literals vs lifetimes.
- `log` added as a dev-dependency (for the dynamic test logger).

## Bugs surfaced (documented in readme known limitations, not fixed)

1. **`1.5em` doesn't compile at all** — `1.5e` starts a Rust float exponent.
   Only affects `em`/`ex` after a decimal point.
2. **`calc(100% - 2px)` → `calc( 100% -2px )`** — the `-` kebab-joiner
   heuristic eats the space, producing invalid `calc()`. Worse: lightningcss
   validation *accepts* it, so it fails silently in the browser.
3. **`input[type="text"]` → `input [ type = "text" ]`** — space before `[`
   turns an attribute selector into a descendant selector. Unfixable by
   heuristic alone (grid line names like `auto [col]` legitimately need that
   space).

Workaround for all three: `r"raw strings"`.

## Possible future work (undecided)

- **Span-based whitespace**: since Rust 1.88, `Span::start()`/`line()`/
  `column()` are stable — the parser could reproduce the actual source
  whitespace instead of guessing from token kinds, fixing quirks 2 and 3 (and
  most spacing weirdness) in one sweep. Quirk 1 is the Rust lexer itself,
  unfixable. Would require bumping MSRV 1.82 → 1.88.
- **`trybuild` compile-fail tests** for the `validation` feature (invalid CSS
  → `compile_error!` with correct span) — currently completely untested;
  stderr snapshots would be somewhat brittle against lightningcss upgrades.

## Verified

- `cargo test` under `--all-features`, `--no-default-features`, and
  `--no-default-features --features dynamic` — all green.
- clippy `--all-features --all-targets` — zero warnings.
- doc tests pass.
