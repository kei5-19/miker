//! Provides primitive types.

#![cfg_attr(not(test), no_std)]

mod mem;

pub mod paging;

pub use mem::*;
