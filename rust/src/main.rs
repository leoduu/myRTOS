
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(debug_assertions, allow(dead_code, unused_unsafe, unused_allocation))]
#![feature(default_alloc_error_handler)]
#![feature(format_args_nl)]
#![feature(const_mut_refs)]

#[macro_use]
extern crate lazy_static;

extern crate alloc;
extern crate cortex_m_rt;

// use panic_halt as _;

mod utilities;
mod drivers;
mod mem;
mod kernel;
mod user;

mod log;

// target architecture
// #[cfg(target_arch = "arm")]
#[path = "arch/arm/cortex-m7/mod.rs"]
pub mod arch;

#[cfg(target_arch = "riscv")]
#[path = "arch/riscv/mod.rs"]
pub mod arch;

// target board
#[cfg(feature = "board_art-pi")]
#[path = "boards/art-pi/mod.rs"]
pub mod board;

