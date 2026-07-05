use std::sync::LazyLock;

use pinkie::css;

fn component1() -> String {
    let style = css! {
        color: red;
    };
    format!(r#"<div class="{style}">first</div>"#)
}

fn component2() -> String {
    let style = css! {
        color: green;
    };
    format!(r#"<div class="{style}">second</div>"#)
}

mod nested {
    use super::*;

    pub fn component3() -> String {
        let style = css! {
            color: blue;
        };
        format!(r#"<div class="{style}">third</div>"#)
    }
}

mod collect_other_file;

#[test]
fn collect_lazy_lock() {
    static STYLES: LazyLock<String> = LazyLock::new(|| stable_order(pinkie::collect()));

    insta::assert_snapshot!(*STYLES, @r"
    .pinkie-11d3b111592ca627{color: rebeccapurple;}
    .pinkie-17e16f7ebf49dd2c{color: green;}
    .pinkie-dc7c21667b5f60bd{color: red;}
    .pinkie-fcf5df288c819b1c{color: blue;}
    ");
}

#[test]
fn collect() {
    let css = stable_order(pinkie::collect());

    let result = format!(
        "<style>\n{css}\n</style>\n{}\n{}\n{}\n{}\n",
        component1(),
        component2(),
        nested::component3(),
        collect_other_file::component4()
    );

    insta::assert_snapshot!(result, @r#"
    <style>
    .pinkie-11d3b111592ca627{color: rebeccapurple;}
    .pinkie-17e16f7ebf49dd2c{color: green;}
    .pinkie-dc7c21667b5f60bd{color: red;}
    .pinkie-fcf5df288c819b1c{color: blue;}
    </style>
    <div class="pinkie-dc7c21667b5f60bd">first</div>
    <div class="pinkie-17e16f7ebf49dd2c">second</div>
    <div class="pinkie-fcf5df288c819b1c">third</div>
    <div class="pinkie-11d3b111592ca627">fourth</div>
    "#);
}

// order is not guaranteed, eh
fn stable_order(s: String) -> String {
    let mut lines = s.lines().collect::<Vec<_>>();
    lines.sort_unstable();
    lines.join("\n")
}
