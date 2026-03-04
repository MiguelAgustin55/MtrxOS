#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

pub use core::arch::asm;
pub use core::fmt::Write;
pub use core::slice;
pub use uefi::prelude::*;
pub use uefi::proto::console::gop::GraphicsOutput;
pub use uefi::proto::console::text::{Color, Key, ScanCode};
pub use uefi::table::runtime::ResetType;

pub mod intro;
pub mod mtrx_gl;
pub mod shell;
pub mod gltest;
pub mod lade;
pub mod buggy;
pub mod raycaster;
pub mod zim;
pub mod sysconf;
pub mod popup;

pub use crate::mtrx_gl::{MtrxGl, Vec2};
