//! Direct3D 9 to Direct3D 11 / DXGI converter.

#![feature(arbitrary_self_types)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(integer_atomics)]
#![feature(rust_2018_preview)]
#![feature(try_trait)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        new_without_default,
        new_ret_no_self,
        not_unsafe_ptr_arg_deref
    )
)]

#[macro_use]
extern crate log;

#[macro_use]
mod macros;

mod error;
pub use self::error::{Error, Result};

pub mod core;

mod dev;
pub use self::dev::Device;

mod d3d11;

mod entry;
pub use self::entry::*;
