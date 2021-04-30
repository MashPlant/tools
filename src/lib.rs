#![feature(try_trait)]

pub mod bitset;
pub mod deterministic_hash;
pub mod fmt;
pub mod impl_try;
pub mod ptr;

pub use deterministic_hash::*;
pub use impl_try::*;
pub use ptr::*;

impl_try!(_ <T> std::ops::Try for P<T>);
impl_try!(_ <T> std::ops::Try for R<T>);

/// Almost identical with `()`, but it implements `Try`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Unit;
impl_try!(Unit);
