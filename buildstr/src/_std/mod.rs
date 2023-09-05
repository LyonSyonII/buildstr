macro_rules! impls {
    ( $($feature:literal => [$($name:ident),*])* ) => {
        $(
            $(
                #[cfg(feature = $feature)]
                mod $name;
            )*
        )*
    };
}

impls! {
    "prelude" => [
        primitive,
        string
    ]
    "extra" => [
        alloc,
        any,
        arch,
        array,
        ascii,
        borrow,
        boxed,
        cell,
        char,
        clone,
        cmp,
        collections,
        convert,
        ffi,
        fmt,
        marker,
        net,
        num,
        ops,
        path,
        process,
        str,
        time
    ]
}