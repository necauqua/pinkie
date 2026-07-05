//! End-to-end test of the `dynamic` feature: `collect()` re-reads this very
//! file at runtime, finds the `css!` invocations and re-parses their content.
#![cfg(all(debug_assertions, feature = "dynamic"))]

use log::{Level, LevelFilter, Log, Metadata, Record};
use pinkie::css;

/// The dynamic path logs a warning and falls back to the static CSS when it
/// fails to read or scan the sources - turn that into a loud test failure.
struct PanicOnWarn;

impl Log for PanicOnWarn {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Warn
    }

    fn log(&self, record: &Record) {
        if record.level() <= Level::Warn {
            panic!("dynamic collection fell back to static: {}", record.args());
        }
    }

    fn flush(&self) {}
}

/// Deliberately tricky content for the source scanner: stray closing braces
/// in comments, string literals and raw strings.
fn tricky() -> pinkie::Style {
    css! {
        // a line comment with a stray brace }
        /* a nested /* block comment */ with a brace } */
        content: "}";
        r"&:hover { color: blue; }"
    }
}

#[test]
fn dynamic_collect_reads_sources() {
    log::set_logger(&PanicOnWarn).unwrap();
    log::set_max_level(LevelFilter::Warn);

    let style = tricky();
    let css = pinkie::collect();

    // the dynamic re-parse of the source must produce exactly
    // the same css as the macro did at compile time
    assert!(
        css.contains(style.css),
        "dynamic output diverged from static\n dynamic: {css}\n  static: {}",
        style.css
    );
}
