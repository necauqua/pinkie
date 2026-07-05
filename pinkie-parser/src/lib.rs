use proc_macro2::{Delimiter, LineColumn, TokenStream, TokenTree};

pub struct CssData {
    pub css: String,
    #[cfg(feature = "spans")]
    pub spans: Vec<(usize, proc_macro2::Span)>,
}

/// Reimplement `TokenStream::to_string`, using span locations to decide the
/// whitespace: two tokens are joined without a space exactly when they were
/// adjacent in the source, and any gap (including newlines) collapses to a
/// single space.
///
/// Additionally, collect spans along with their offsets in the output string
/// for pretty nice errors for the amount of effort it took.
pub fn parse(input: TokenStream) -> CssData {
    let mut data = CssData {
        css: String::new(),
        #[cfg(feature = "spans")]
        spans: Vec::new(),
    };
    parse_recursive(input, &mut data, &mut None);
    data
}

fn parse_recursive(input: TokenStream, out: &mut CssData, prev_end: &mut Option<LineColumn>) {
    // push a single space if there was any gap between the previous token
    // and one starting at `start` in the source
    let gap = |out: &mut CssData, prev_end: &Option<LineColumn>, start: LineColumn| {
        if prev_end.is_some_and(|prev| prev != start) {
            out.css.push(' ');
        }
    };

    for tree in input {
        match tree {
            TokenTree::Punct(punct) => {
                gap(out, prev_end, punct.span().start());

                #[cfg(feature = "spans")]
                out.spans.push((out.css.len(), punct.span()));

                out.css.push(punct.as_char());
                *prev_end = Some(punct.span().end());
            }
            TokenTree::Ident(ident) => {
                gap(out, prev_end, ident.span().start());

                #[cfg(feature = "spans")]
                out.spans.push((out.css.len(), ident.span()));

                out.css.push_str(ident.to_string().trim_start_matches("r#"));
                *prev_end = Some(ident.span().end());
            }
            TokenTree::Group(group) => {
                let (open, close) = match group.delimiter() {
                    Delimiter::Brace => ('{', '}'),
                    Delimiter::Parenthesis => ('(', ')'),
                    Delimiter::Bracket => ('[', ']'),
                    Delimiter::None => {
                        parse_recursive(group.stream(), out, prev_end);
                        continue;
                    }
                };

                gap(out, prev_end, group.span_open().start());

                #[cfg(feature = "spans")]
                out.spans.push((out.css.len(), group.span_open()));

                out.css.push(open);
                *prev_end = Some(group.span_open().end());

                parse_recursive(group.stream(), out, prev_end);

                gap(out, prev_end, group.span_close().start());

                #[cfg(feature = "spans")]
                out.spans.push((out.css.len(), group.span_close()));

                out.css.push(close);
                *prev_end = Some(group.span_close().end());
            }
            TokenTree::Literal(lit) => {
                gap(out, prev_end, lit.span().start());

                #[cfg(feature = "spans")]
                out.spans.push((out.css.len(), lit.span()));

                let str = lit.to_string();
                if str.starts_with('r') {
                    // only usage of syn, can maybe drop?.
                    if let Ok(str) = syn::parse_str::<syn::LitStr>(&str) {
                        // replace newlines to keep simple span mapping
                        out.css.push_str(&str.value().replace(['\n', '\r'], " "));
                        *prev_end = Some(lit.span().end());
                        continue;
                    }
                }
                out.css.push_str(&str);
                *prev_end = Some(lit.span().end());
            }
        }
    }
}
