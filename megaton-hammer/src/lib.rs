//! # Megaton Hammer
//!
//! Welcome to the Fire Temple. I hope you're equipped with the Megaton Hammer.
//! Because it's time to hit some Rusty Switches! (I'm so sorry.)
//!
//! This crate's goal is to give the user all the low-level primitives needed
//! to interact with the Switch OS. It provides primitives for IPC, TLS,
//! syscalls.
//!
//! Note that this crate does not contain actual IPC definitions. This is left
//! for other crates (I'll be making one that uses SwIPC to create all the IPC).
//!
//! # Why another toolchain ?
//!
//! Because I firmly believe that as much stuff as possible should be written in
//! Rust. My first attempt at writing a rust toolchain reused libtransistor -
//! however, that proved to be more of a hassle than simply reimplementing
//! everything myself.
// TODO: I shouldn't need either of those, in an ideal world.
#![feature(asm, proc_macro, cfg_target_vendor, global_asm, unicode, ptr_internals, naked_functions, alloc, allocator_api, core_intrinsics, const_fn, align_offset, nonzero)]
#![cfg_attr(feature = "crt0", feature(lang_items, compiler_builtins_lib))]
#![no_std]
#[cfg(feature = "std")]
extern crate std;

extern crate spin;
extern crate byteorder;
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

//#[macro_use]
//extern crate failure_derive;

//extern crate bitfield_register;
//extern crate bitfield_register_macro;

extern crate bit_field;
extern crate alloc;

#[macro_use]
extern crate static_assertions;

extern crate arrayvec;
#[macro_use]
extern crate enum_primitive;

pub mod ipc;
pub mod tls;
pub mod kernel;
pub mod ipcdefs;
mod log_impl;

mod utils;

pub mod loader;

pub mod error;
