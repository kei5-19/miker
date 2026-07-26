//! Utility library widely used in MIKer.

#![cfg_attr(not(test), no_std)]
#![allow(clippy::undocumented_unsafe_blocks)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod font_data;

pub mod acpi;
pub mod apic;
pub mod asmfunc;
pub mod bitfield;
pub mod buffer;
pub mod descriptor;
pub mod driver;
pub mod elf;
pub mod graphics;
pub mod interrupt;
pub mod paging;
pub mod pci;
pub mod screen;
pub mod sync;

#[cfg(feature = "alloc")]
pub mod collections;

#[cfg(feature = "alloc")]
pub mod error;

#[cfg(feature = "alloc")]
pub mod hash;

pub use macros::*;
