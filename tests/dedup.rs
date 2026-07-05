use pinkie::css;

fn first() -> pinkie::Style {
    css! {
        color: teal;
    }
}

fn second() -> pinkie::Style {
    css! {
        color: teal;
    }
}

#[test]
fn identical_styles_dedup() {
    let (a, b) = (first(), second());
    assert_eq!(a.class, b.class);
    assert_eq!(a.css, b.css);

    // both submitted to the inventory, but only one rule is emitted
    let css = pinkie::collect();
    assert_eq!(css.matches(a.class).count(), 1, "collected: {css}");
}

#[test]
fn class_name_format() {
    let class = first().class;
    let hash = class.strip_prefix("pinkie-").expect("default prefix");
    assert_eq!(hash.len(), 16, "full xxh64 hash");
    assert!(hash.bytes().all(|b| b.is_ascii_hexdigit()));
}
