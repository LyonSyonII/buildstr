# Types not implemented
The types listed here have no way to be built by themselves, and are therefore not implemented by the buildstr library, or are unstable at the moment.

Trait objects are not yet implemented, except for `Debug` and `Display`.

# Nonsensical
- `std::convert::Infallible`
  As the type's description says, a value of this enum can never be constructed, so it makes no sense to implement it.

- `std::env::Args`
  The struct collects all arguments passed to the program, and all constructors are private.
- `std::env::ArgsOs`
  Same as `Args`.
- `std::env::Vars`
- `std::env::VarsOs`
  
- `std::mem::MaybeUninit<T>`  
  There is no way to know if a MaybeUninit is initialized or not only with itself, so it makes no sense to implement it for all contexts.  
  If you have a struct with this type, implement `BuildStr` manually, checking if it's initialized or not.

- `std::net::TcpListener`
  The current `local_addr` can be obtained, but it returns a `Result`, having the possibility to panic at runtime.  
  The `TcpListener::bind` method also returns a `Result`, making it unreliable.
- `std::net::TcpStream`  
  Same as TcpListener.
- `std::net::UpdSocket`  
  Same as TcpListener.

- `core::ptr::NonNull<T>`
  All operations on `NonNull` are unsafe, so `BuildStr` cannot account for all situations.  
  Instead of implementing for the "happy path" (valid pointer), it makes more sense if each user implements it depending on the context.

## Opaque
- `std::alloc::LayoutError`
- `std::arch::*::__*`
  All types except `std::arch::x86_64::CpuidResult`
- `std::any::TypeId`
- `std::backtrace::Backtrace`
- `std::backtrace::BacktraceStatus`
- `std::cell::BorrowError`
- `std::cell::BorrowMutError`
- `std::cell::Ref<'_, T>`
- `std::cell::RefMut<'_, T>`
- `std::fmt::DebugList`
- `std::fmt::DebugMap`
- `std::fmt::DebugSet`
- `std::fmt::DebugStruct`
- `std::fmt::DebugTuple`
- `std::fmt::Formatter`
- `std::fmt::Write`

- `std::fs::DirBuilder`
  It only has one field `recursive`, but it's private.

- `std::future::PollFn<F>`  
  Can be trivially built with `std::future::poll_fn`, but the function cannot be obtained at runtime.

- `std::time::Instant`  
  The only method available to generate an `Instant` is `Instant::now()`, which is obviously not what you want.

- `std::time::SystemTime`
  Same as `std::time::Instant`.

- `std::mem::Discriminant<T>`
  Can be obtained trivially with `core::mem::discriminant`, but by definition the struct is opaque, so the value cannot be obtained at runtime.  
  It is undefined behavior to transmute between `DiscriminantKind::Discriminant` and `mem::Discriminant`.

- `std::num::ParseFloatError`
  The builder is private and the type has no builder method.

- `std::num::TryFromIntError`

- `std::panic::Location`
  The constructor is internal, so it can only be built by the compiler.

- `std::panic::PanicInfo`
  Same as `panic::Location`.

- `std::process::Child`
  The constructors's argument `handle` is private, and there is no other way to build a `Child`.

- `std::process::ChildStderr`
- `std::process::ChildStdin`
- `std::process::ChildStdout`
- `std::str::ParseBoolError`
- `std::str::Utf8Error`
- `std::string::FromUtf16Error`
- `std::string::FromUtf8Error`


## Iterator
Building the iterator would need to consume the source, which is not possible with a `&self` reference.
- `std::env::SplitPaths<'_>`
- `std::error::Source<'_>`
- `std::iter::*`
- `std::net::Incoming<'_>`
- `std::net::IntoIncoming<'_>`
- `std::slice::*`

## Unstable
- `std::ascii::Char`
- `std::convert::FloatToInt`
- `std::cell::LazyCell<T>`
- `std::cell::SyncUnsafeCell<T>`
- `std::ffi::NonZero_c_XXX`
- `std::ffi::VaList<'_, '_>`
- `std::ffi::VaListImpl<'_, '_>`
- `std::ffi::c_void`
- `std::fs::FileTimes`

- `std::future::Ready<T>`  
  Can be trivially built with `std::future::ready`, but the value cannot be obtained at runtime until `Ready::into_inner()` is stable.

- `std::mem::Assume`
- `std::net::Ipv6MulticastScope`
- `std::num::Saturating<T>`
- `std::ops::GeneratorState<Y, R>`
- `std::ops::Yeet<T>`
- `std::panic::BacktraceStyle`
- `std::ptr::DynMetadata<T>`

## Needs mutable
These types need to be mutable to be built.
- `&dyn std::convert::AsMut<T>`

## Traits that can't be built into a trait object
- `std::convert::From`
- `std::convert::Into`
- `std::convert::TryFrom`
- `std::convert::TryInto`
- `std::default::Default`

## Deprecated
- `core::hash::SipHasher`