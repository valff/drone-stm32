//! [Drone] implementation for ARM Cortex-M microcontrollers.
//!
//! # Installation
//!
//! Instructions will be given for Debian-based Linux systems.
//!
//! Install the following packages:
//!
//! ```sh
//! $ sudo apt-get install build-essential cmake libusb-1.0-0 libusb-1.0-0-dev \
//!   pandoc gcc-arm-none-eabi gdb-arm-none-eabi qemu-system-arm qemu-user
//! ```
//!
//! Copy [udev rules][rules.d] for ST-Link programmer to the
//! `/etc/udev/rules.d/`, and run the following commands:
//!
//! ```sh
//! $ sudo udevadm control --reload-rules
//! $ sudo udevadm trigger
//! ```
//!
//! [OpenOCD] is required. It is recommended to install it from the source,
//! because repository package is outdated and doesn't contain configuration for
//! newer chips and boards.
//!
//! [Drone]: https://github.com/drone-os/drone
//! [OpenOCD]: http://openocd.org/
//! [rules.d]: https://github.com/texane/stlink/tree/master/etc/udev/rules.d

#![feature(alloc)]
#![feature(asm)]
#![feature(associated_type_defaults)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(exhaustive_patterns)]
#![feature(futures_api)]
#![feature(generators)]
#![feature(generator_trait)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(marker_trait_attr)]
#![feature(naked_functions)]
#![feature(never_type)]
#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![feature(range_contains)]
#![feature(untagged_unions)]
#![no_std]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(
  clippy::cast_possible_truncation,
  clippy::doc_markdown,
  clippy::enum_glob_use,
  clippy::inline_always,
  clippy::module_name_repetitions,
  clippy::precedence,
  clippy::shadow_unrelated,
  clippy::use_self
)]
#![cfg_attr(test, feature(allocator_api, allocator_internals))]
#![cfg_attr(test, default_lib_allocator)]

extern crate alloc;

#[macro_use]
pub mod itm;

pub mod cpu;
pub mod drv;
pub mod fib;
pub mod map;
pub mod prelude;
pub mod reg;
pub mod stack_adapter;
pub mod sv;
pub mod thr;

mod lang_items;

pub use drone_cortex_m_macros::{sv, vtable};

#[prelude_import]
#[allow(unused_imports)]
use crate::prelude::*;

#[cfg(test)]
drone_core::heap! {
  struct Heap;
  size = 0x40000;
  pools = [
    [0x4; 0x4000],
    [0x20; 0x800],
    [0x100; 0x100],
    [0x800; 0x20],
  ];
}

#[cfg(test)]
#[global_allocator]
static mut GLOBAL: Heap = Heap::new();
