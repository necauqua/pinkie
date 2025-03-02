# <img align="right" src="pinkie.png" alt="CSS is shiny." title="CSS is shiny."> pinkie
[![Cargo](https://img.shields.io/crates/v/pinkie.svg)](https://crates.io/crates/pinkie)

Pinkie is a simple scoped CSS ~~concatenator~~ generator for Rust.

It does not do _any_ templating, the only feature over static .css files it
provides is "per-component" (per-macro-invocation, technically) scoping.

It's implemented as a macro, `css!`, which allows you to write bits of CSS
throughout your codebase and have them all concatenated into a single string
under content-addressable (they have hashes) classes.

This relies on CSS Nesting - so it is highly recommended to additionally
transpile it by something like `lightningcss`.

Pinkie does not do any CSS parsing on its own, it just translates Rust tokens
into a string with a few CSS-specific heuristics about whitespaces.

In case those fail, you can also include CSS verbatim using `r"raw strings"`,
their content will be included as is.

Sadly, it's close to impossible to reliably collect macro invocations across
a codebase entirely during the build time, so it uses the excellent `inventory`
crate to collect and concatenate all generated bits of CSS at runtime.

It's a startup-only thing, so it's not a big deal, and if you use
`lightningcss` to minimize them they say it takes 4ms to do that for the
entirety of Bootstrap, which is an okay tradeoff for having pinkie exist and be
working.

## Example
```rust
use std::sync::LazyLock;
use maud::{html, Markup};
use pinkie::css;

/// A maud component styled with pinkie.
fn cool_link() -> Markup {
    let style = css! {
        font-family: mono;

        --dot-width: 0;
        &:hover {
            --dot-width: 1ch;
        }
        > .dot {
            display: inline-flex;
            text-decoration: inherit;
            width: var(--dot-width);
            overflow: hidden;
            transition: width 0.1s;
        }
    };
    html! {
        a .(style) href="https://necauq.ua" {
            span { "necauq" }
            span.dot { "." }
            span { "ua" }
        }
    }
}

// `lightningcss` is strongly recommended as
// pinkie uses CSS Nesting for scoping
fn minimize(css: String) -> String {
    css
}

fn main() {
    // You'd need to store them somewhere
    static STYLES: LazyLock<String> = LazyLock::new(|| minimize(pinkie::collect()));

    let btn = cool_link().into_string();

    // And then can serve them in any way
    println!("<style>{}</style>", *STYLES);
    println!("{btn}");
}
```

## Cargo features
- `validation` (default) - enables (some) compile time CSS validation by
  `lightningcss`. Slows down the first proc macro compilation by bringing it.
- `location` (default) - enables storing the location of `css!` invocations,
  useful for debugging (the hash collision panic will use it).
- `dynamic` - changes the style collection method to trying to read Rust
  sources (depends on `location`) at runtime, allowing one to implement style
  hot-reloading during development, as Rust recompilation speeds are a thing.
  Only does anything when `debug_assertions` are enabled.

If you want custom class prefix you can have it by setting
`PINKIE_CSS_CLASS_PREFIX` environment variable during compilation.
You might want to do `cargo clean` once for it to take effect.

And you can have the prefix be of any length, non-minimized, literally any
compression algorithm will take care of it lol.

## Inspiration
A bit obviously inspired by `maud` and works great with it.

In case you didn't know, MPAs actually work extremely well for most use-cases
in modern browsers, especially if you write your web-server in a real language,
like Rust or Go :)
