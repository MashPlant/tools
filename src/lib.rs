#![feature(try_trait)]
#![feature(try_trait_v2)]
#![feature(control_flow_enum)]
#![feature(new_uninit)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod bitset;
pub mod deterministic_hash;
pub mod fmt;
pub mod impl_try;
pub mod ptr;

pub use deterministic_hash::*;
pub use impl_try::*;
pub use ptr::*;

impl_residual!(_ <T> core::ops::FromResidual for P<T>);
impl_try!(_ <T> core::ops::Try for P<T>);
impl_residual!(_ <T> core::ops::FromResidual for R<T>);
impl_try!(_ <T> core::ops::Try for R<T>);

/// Almost identical with `()`, but it implements `Try`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Unit;
impl_try!(Unit);
