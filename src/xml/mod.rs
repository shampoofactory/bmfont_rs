//! XML format operations.

mod load;
mod store;

pub use load::{from_bytes, from_bytes_ext, from_reader, from_reader_ext, from_str, from_str_ext};
pub use store::{to_string, to_vec, to_writer};
