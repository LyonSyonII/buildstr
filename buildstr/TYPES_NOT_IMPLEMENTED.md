# Types not implemented
The types listed here have no way to be built by themselves, and are therefore not implemented by the buildstr library, or are unstable at the moment.

# Nonsensical
- `core::mem::MaybeUninit<T>`  
  There is no way to know if a MaybeUninit is initialized or not only with itself, so it makes no sense to implement it for all contexts.  
  If you have a struct with this type, implement `BuildStr` manually, checking if it's initialized or not.

## Opaque
- `std::any::TypeId`
- `std::backtrace::Backtrace`
- `std::cell::Ref<'_, T>`
- `std::cell::RefMut<'_, T>`

- `std::future::PollFn<F>`  
  Can be trivially built with `std::future::poll_fn`, but the function cannot be obtained at runtime.

- `std::time::Instant`  
  The only method available to generate an `Instant` is `Instant::now()`, which is obviously not what you want.

- `std::time::SystemTime`
  Same as `std::time::Instant`.

- `core::mem::Discriminant<T>`
  Can be obtained trivially with `core::mem::discriminant`, but by definition the struct is opaque, so the value cannot be obtained at runtime.  
  It is undefined behavior to transmute between `DiscriminantKind::Discriminant` and `mem::Discriminant`.

## Iterator
Building the iterator would need to consume the source, which is not possible with a `&self` reference.
- `std::error::Source<'_>`
- `std::iter::*`
- `std::net::Incoming<'_>`
- `std::net::IntoIncoming<'_>`

## Unstable
- `std::ascii::Char`
- `std::cell::LazyCell<T>`
- `std::cell::SyncUnsafeCell<T>`
- `std::ffi::NonZero_c_XXX`
- `std::ffi::VaList<'_, '_>`
- `std::ffi::VaListImpl<'_, '_>`
- `std::ffi::c_void`

- `std::future::Ready<T>`  
  Can be trivially built with `std::future::ready`, but the value cannot be obtained at runtime until `Ready::into_inner()` is stable.

- `std::mem::Assume`
- `std::net::Ipv6MulticastScope`

## Deprecated
- `core::hash::SipHasher`