//!
//! # Overview
//! _EARLY WIP_
//! A specialized pure rust library for generating images containing hex tiles
//! with an efficient memory layout.
//!

// NOTE Warnings are automatically rejected by CI when trying to merge into main
pub mod api;
pub(crate) mod rasterization;
#[warn(missing_docs, rust_2018_idioms, rust_2021_compatibility)]
pub mod types;

pub mod prelude {
    pub use crate::types::{Line, Point};
}

#[cfg(test)]
pub(crate) mod tests_prelude {
    use image::Rgba;

    pub(crate) const ARRAY2_ZERO: [u32; 2] = [0; 2];
    pub(crate) const TEST_SIZE: [u32; 2] = [8, 8];
    pub(crate) const COLOR_WHITE_RGBA: Rgba<u8> = Rgba {
        0: [255, 255, 255, 255],
    };
}
