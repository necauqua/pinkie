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
            // account for `.dummy{` and columns being 1-based
            let offset = (loc.column as usize).saturating_sub(8);

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

    let hash = xxh64(css.as_bytes(), 0);
    let class = format!("{prefix}{hash:016x}");

    let location = if cfg!(feature = "location") {
        quote! {
            location: ::pinkie::Location {
                file: file!(),
                line: line!() as usize,
            },
        }
    } else {
        quote!()
    };

    quote! {{
        // make cargo track the env var and rebuild when it changes
        const _: ::core::option::Option<&str> = ::core::option_env!("PINKIE_CSS_CLASS_PREFIX");
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
