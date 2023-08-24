# Types not implemented
The types listed here have no way to be built by themselves, and are therefore not implemented by the buildstr library, or are unstable at the moment.


## Opaque

- `std::any::TypeId`
- `std::backtrace::Backtrace`
- `std::cell::Ref<'_, T>`
- `std::cell::RefMut<'_, T>`

- `std::time::Instant`  
  The only method available to generate an `Instant` is `Instant::now()`, which is obviously not what you want.

- `std::time::SystemTime`
  Same as `std::time::Instant`.

## Unstable

- `std::ascii::Char`
- `std::cell::LazyCell<T>`
- `std::cell::SyncUnsafeCell<T>`