//! .areal binary file format — header, sections, footer.

pub mod file;
pub mod footer;
pub mod header;
pub mod sections;

pub use file::{ArealReader, ArealWriter};
