#![feature(unique)]
//! Rust bindings for [libffi](https://sourceware.org/libffi/).
//!
//! # Purpose
//!
//! Libffi provides two main facilities:
//!
//!   - Assembling *calls* to functions dynamically.
//!   - Creating *closures* that can be called as ordinary C functions.
//!
//! The former is useful mostly for implementing FFIs for untyped
//! languages; this library provides some support in the
//! [`middle`](middle/index.html) and [`low`](low/index.html) layers,
//! but I’m not sure how useful it is. The latter can be used to
//! interface between higher-order languages and C by making closures
//! from the higher-order language callable as C function pointers. In
//! Rust, this means that we can now, for example, pass a lambda as a
//! callback to a C function.
//!
//! Most users are likely interested in the [`high`](high/index.html)
//! layer, which provides the easiest interface to the closure facility.
//!
//! # Organization
//!
//! This library is organized in four layers, each of which attempts to
//! provide more safety and a simpler interface than the next layer
//! down. From top to bottom:
//!
//!   - The [`high`](high/index.html) layer provides safe(?) and
//!     automatic marshalling of Rust closures into C function pointers.
//!   - The [`middle`](middle/index.html) layer provides memory-managed
//!     abstractions for assembling calls and closures, but is unsafe
//!     because it doesn’t check argument types.
//!   - The [`low`](low/index.html) layer makes no attempts at safety,
//!     but provides a more idiomatically “Rusty” API than the underlying
//!     C library.
//!   - The [`raw`](raw/index.html) layer is a direct mapping of the
//!     C libffi library into Rust, generated by [Rust
//!     Bindgen](https://github.com/crabtw/rust-bindgen).
//!
//! It should be possible to use any layer without dipping into lower
//! layers (and it will be considered a bug to the extent that it
//! isn’t).
//!
//! # Example
//!
//! In this example, we convert a Rust lambda containing a free variable
//! into an ordinary C code pointer. The type of `fun` below is
//! `extern "C" fn(u64, u64) -> u64`.
//!
//! ```
//! use libffi::high::Closure2;
//!
//! let x = 5u64;
//! let f = |y: u64, z: u64| x + y + z;
//!
//! let closure = Closure2::new(&f);
//! let fun     = closure.code_ptr();
//!
//! assert_eq!(18, fun(6, 7));
//! ```

extern crate libc;

/// Unwrapped definitions imported from the C library (via bindgen).
pub mod raw;

pub mod high;
pub mod middle;
pub mod low;
