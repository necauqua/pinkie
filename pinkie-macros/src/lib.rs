use std::borrow::Cow;

#[cfg(feature = "validation")]
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use quote::quote;
use xxhash_rust::xxh64::xxh64;

#[proc_macro]
pub fn css(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data = pinkie_parser::parse(input.into());
    let css = data.css;

    #[cfg(feature = "validation")]
    if let Err(e) = StyleSheet::parse(&format!(".dummy{{{css}}}"), ParserOptions::default()) {
        let err = format!("css error: {}", e.kind);
        if let Some(loc) = e.loc {
            let offset = loc.column as usize - 8; // account for `.dummy{` and columns being 1-based

            // println!("{css}");
            // let mut prev = 0;
            // let mut highlighted = false;
            // for (pos, _) in &data.spans {
            //     if *pos >= offset && !highlighted {
            //         highlighted = true;
            //         print!("{: >fill$}\x1b[1;31m*\x1b[0m", "", fill = pos - prev);
            //     } else {
            //         print!("{: >fill$}^", "", fill = pos - prev);
            //     }
            //     prev = *pos + 1; // account for ^ taking up a space
            // }
            // println!();

            let span = data
                .spans
                .into_iter()
                .find_map(|(pos, span)| (offset <= pos).then_some(span));
            if let Some(span) = span {
                return quote::quote_spanned!(span => compile_error!(#err)).into();
            }
        }
        return quote!(compile_error!(#err)).into();
    };

    let prefix = std::env::var("PINKIE_CSS_CLASS_PREFIX")
        .map(Cow::Owned)
        .unwrap_or("pinkie-".into());

    let hash = &format!("{:08x}", xxh64(css.as_bytes(), 0))[0..8];
    let class = format!("{prefix}{hash}");

    let (line, location) = if cfg!(feature = "location") {
        (
            quote!(
                const LINE: usize = line!() as usize;
            ),
            quote! {
                location: ::pinkie::Location {
                    file: file!(),
                    line: LINE,
                },
            },
        )
    } else {
        (quote!(), quote!())
    };

    quote! {{ #line;
        const STYLE: ::pinkie::Style = ::pinkie::Style {
            class: #class,
            css: #css,
            #location
        };
        ::pinkie::__submit!(STYLE);
        STYLE
    }}
    .into()
}
