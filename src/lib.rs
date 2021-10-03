//!
//! # Overview
//! _EARLY WIP_
//! A specialized pure rust library for generating images containing hex tiles
//! with an efficient memory layout.
//!

// NOTE Warnings are automatically rejected by CI when trying to merge into main
#![warn(
    clippy::all,
    missing_docs,
    rust_2018_idioms,
    rust_2021_compatibility,
    missing_debug_implementations
)]

pub(crate) mod rasterization;
pub mod types;

/// Public re-exports module
pub mod prelude {
    pub use crate::types::{Line, Point};

    #[cfg(test)]
    pub(crate) use test::*;

    #[cfg(test)]
    #[allow(dead_code)]
    mod test {
        use crate::image::Rgba;
        pub(crate) const TEST_SIZE: u32 = 8;
        pub(crate) const COLOR_WHITE_RGBA: Rgba<u8> = Rgba {
            0: [255, 255, 255, 255],
        };
    }
}

/// Re-export of the Image crate.
pub mod image {
    // TODO export only the necessary types
    pub use ::image::*;
}
