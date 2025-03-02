use proc_macro2::{Delimiter, TokenStream, TokenTree};

pub struct CssData {
    pub css: String,
    #[cfg(feature = "spans")]
    pub spans: Vec<(usize, proc_macro2::Span)>,
}

/// Naively reimplement `TokenStream::to_string` with a few tweaks for
/// CSS-significant (lack of) whitespace in a couple of specific places.
///
/// Additionally, collect spans along with their offsets in the output string
/// for pretty nice errors for the amount of effor it took.
pub fn parse(input: TokenStream) -> CssData {
    let mut data = CssData {
        css: String::new(),
        #[cfg(feature = "spans")]
        spans: Vec::new(),
    };
    parse_recursive(input, &mut data);
    data
}

fn parse_recursive(input: TokenStream, out: &mut CssData) {
    let mut input = input.into_iter().peekable();

    while let Some(tree) = input.next() {
        match tree {
            TokenTree::Punct(punct) => {
                #[cfg(feature = "spans")]
                let pos = out.css.len();

                let ch = punct.as_char();
                out.css.push(ch);
                if !matches!(ch, '.' | '#' | '-' | '@' | ':' | '&') {
                    #[cfg(feature = "spans")]
                    out.spans.push((pos, punct.span()));

                    out.css.push(' ');
                }
            }
            TokenTree::Ident(ident) => {
                #[cfg(feature = "spans")]
                out.spans.push((out.css.len(), ident.span()));

                out.css.push_str(ident.to_string().trim_start_matches("r#"));

                // allow kebab-case and pseudo-classes
                let next = input.peek();
                if !matches!(next, Some(TokenTree::Punct(p)) if matches!(p.as_char(), '-' | ':')) {
                    out.css.push(' ');
                }
            }
            TokenTree::Group(group) => {
                let (open, close) = match group.delimiter() {
                    Delimiter::Brace => ("{ ", "} "),
                    Delimiter::Parenthesis => ("( ", ") "),
                    Delimiter::Bracket => ("[ ", "] "),
                    Delimiter::None => ("", ""),
                };

                #[cfg(feature = "spans")]
                out.spans.push((out.css.len(), group.span_open()));

                out.css.push_str(open);
                parse_recursive(group.stream(), out);

                #[cfg(feature = "spans")]
                out.spans.push((out.css.len(), group.span_close()));

                out.css.push_str(close);
            }
            TokenTree::Literal(lit) => {
                #[cfg(feature = "spans")]
                out.spans.push((out.css.len(), lit.span()));

                let str = lit.to_string();
                if str.starts_with('r') {
                    // only usage of syn, can maybe drop?.
                    if let Ok(str) = syn::parse_str::<syn::LitStr>(&str) {
                        // replace newlines to keep simple span mapping
                        let str = str.value().replace(|ch| ch == '\n' || ch == '\r', " ");
                        out.css.push_str(&str);
                        out.css.push(' ');
                        continue;
                    }
                }
                out.css.push_str(&str);
                out.css.push(' ');
            }
        }
    }
}
