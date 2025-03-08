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
