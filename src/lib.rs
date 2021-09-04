#![warn(missing_docs)]
/*!


*/
mod builder;
mod charset;
mod error;
mod font;
mod parse;
mod tagged_attributes;

#[cfg(test)]
mod tests;

pub mod binary;
pub mod text;
#[cfg(feature = "xml")]
pub mod xml;

pub use charset::*;
pub use error::{Error, Result};
pub use font::{Char, Chnl, Common, Font, Info, Kerning, Packing, Padding, Spacing};
