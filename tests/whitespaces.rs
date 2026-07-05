use pinkie::css;

macro_rules! test {
    ($title:ident -> { $($slurped:tt)* } @$snap:literal) => {
        #[test]
        fn $title() {
            insta::assert_snapshot!(
                css! { $($slurped)* }.css,
                @$snap
            )
        }
    };
    ($title:ident -> { $($slurped:tt)*} $head:tt $($rest:tt)*) => {
        test! { $title -> { $($slurped)* $head } $($rest)* }
    };
    ($title:ident -> $($slurp:tt)*) => {
        test! { $title -> {} $($slurp)* }
    };
}

test! { class ->
    .class {}

    @".class { }"
}

test! { id ->
    #id {}

    @"#id { }"
}

test! { property ->
    --property: value;
    --kebab-prop: 123;

    @"--property:value ; --kebab-prop:123 ;"
}

test! { kebab ->
    custom-element {}

    @"custom-element { }"
}

test! { media ->
    @media (min-width: 600px) {}

    @"@media( min-width:600px ) { }"
}

test! { pseudoclass ->
    :root {}
    a:hover {}

    @":root { } a:hover { }"
}

test! { nesting_op ->
    & {}
    &.class {}
    &:hover {}

    @"&{ } &.class { } &:hover { }"
}

test! { function_call ->
    color: var(--property);

    @"color:var( --property ) ;"
}

test! { suffixes ->
    font-size: 1rem;
    margin: 15px;
    padding: 25%;

    @"font-size:1rem ; margin:15px ; padding:25% ;"
}

test! { hex_colors ->
    color: #fff;
    border-color: #c0ffee;
    background: #123456;
    outline-color: #00ff00;

    @"color:#fff ; border-color:#c0ffee ; background:#123456 ; outline-color:#00ff00 ;"
}

test! { negative_values ->
    margin: -5px;
    top: -1.5rem;

    @"margin:-5px ; top:-1.5rem ;"
}

test! { time_units ->
    transition: width 0.1s ease-in-out;

    @"transition:width 0.1s ease-in-out ;"
}

test! { strings ->
    content: "hello world";

    @r#"content:"hello world" ;"#
}

test! { raw_verbatim ->
    r"&:hover { color: blue; }"

    @"&:hover { color: blue; }"
}

test! { important ->
    width: 0 !important;

    @"width:0 ! important ;"
}

// documents a known quirk: css `calc()` requires spaces around `-`,
// but `-` is treated as a kebab-case joiner - use a raw string for calc
test! { calc_minus ->
    width: calc(100% - 2px);

    @"width:calc( 100% -2px ) ;"
}

// documents a known quirk: the space before `[` makes this a descendant
// selector instead of an attribute selector
test! { attribute_selector ->
    input[type="text"] {}

    @r#"input [ type = "text" ] { }"#
}
