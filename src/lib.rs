//! Fast, high-fidelity OpenType parser.

#![no_std]

pub mod avar;
pub mod cmap;
pub mod fvar;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod maxp;
pub mod os2;
pub mod parse;
pub mod post;
pub mod types;
pub mod vhea;
pub mod vmtx;

mod font;

pub use font::*;

/// Helper module for common parsing imports.
mod parse_prelude {
    pub use super::parse::*;
    pub use super::types::*;
}
