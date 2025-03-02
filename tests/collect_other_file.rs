use pinkie::css;

pub fn component4() -> String {
    let style = css! {
        color: rebeccapurple;
    };
    format!(r#"<div class="{style}">fourth</div>"#)
}
