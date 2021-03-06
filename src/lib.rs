//!
//! # Overview
//! _EARLY WIP_
//! A specialized pure rust library for generating images containing hex tiles
//! with an efficient memory layout.
//!

#![warn(
    clippy::all,
    missing_docs,
    rust_2018_idioms,
    rust_2021_compatibility,
    missing_debug_implementations
)]

// NOTE Warnings are automatically rejected by CI when trying to merge into main
pub mod api;
pub(crate) mod rasterization;
pub mod types;

/// Public re-exports module
pub mod prelude {
    pub use crate::types::{Line, Point};
}

#[cfg(test)]
pub(crate) mod tests_prelude {
    use image::Rgba;

    pub(crate) const TEST_SIZE: u32 = 8;
    pub(crate) const COLOR_WHITE_RGBA: Rgba<u8> = Rgba {
        0: [255, 255, 255, 255],
    };
}
