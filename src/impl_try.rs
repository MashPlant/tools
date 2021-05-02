/// Panic in debug mode, UB (call `core::hint::unreachable_unchecked`) in release mode.
#[macro_export] macro_rules! debug_panic {
  ($($arg:tt)*) => (if cfg!(debug_assertions) {
    panic!($($arg)*);
  } else {
    unsafe { core::hint::unreachable_unchecked()}
  })
}

/// `debug_panic` when "debug-panic" feature is enabled; panic otherwise.
#[inline(never)]
#[cold]
#[track_caller]
pub fn try_failed() -> ! {
  #[cfg(feature = "debug-panic")]
  debug_panic!("try failed");
  #[cfg(not(feature = "debug-panic"))]
  panic!("try failed");
}

/// Implement `core::ops::Try` for an arbitrary type,
/// so that the `?` operator can be used on a `core::option::Option` value in a function that returns this type.
/// When the option value is `None`, the `?` operator will call `try_failed`, which will panic;
/// when the option value is `Some`, the value the `?` operator will return the wrapped value.
#[macro_export] macro_rules! impl_try {
  ($ty: ty) => { impl_try!(_ core::ops::Try for $ty); };
  (_ $($arg:tt)*) => { // _ is necessary to distinguish from ty
    impl $($arg)* {
      type Ok = Self;
      type Error = core::option::NoneError;
      fn into_result(self) -> core::result::Result<Self::Ok, Self::Error> { Ok(self) }
      #[inline(always)]
      #[track_caller]
      fn from_error(_: Self::Error) -> Self { $crate::impl_try::try_failed() }
      fn from_ok(v: Self::Ok) -> Self { v }
    }
  };
}
