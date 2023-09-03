# Types not exact
The types listed here have no way to be built exactly as they are, but can be implemented with some tricks.

Most of these types will not be exactly the same as the original, but they should be close enough for the purpose of buildstr.

All error-like types are built by forcing the error to happen.

If you have any issues with these types, consider implementing them yourself.

- `std::array::TryFromSliceError`
- `std::char::CharTryFromError`

- `core::fmt::Arguments<'_>`
  The `format_args!` macro is internal to the compiler, `buildstr` will convert the arguments to a formatted string and then call `format_args` with a literal.

- `core::hash::BuildHasherDefault<H>`
  This is a type rarely stored, but instead used as BuildHasher in `HashMap` and `HashSet`, the current implementation uses Default::default() to get an instance, which could have consequences I don't know about.