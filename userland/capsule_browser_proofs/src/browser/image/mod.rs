// NONOS Operating System (AGPL-3.0-or-later)
// The decoder under test is pub(super), so it is only pulled in for the tests
// that exercise it; that keeps the non-test build free of an unused import.
#[cfg(test)]
#[path = "../../../../base64/src/decode.rs"]
pub mod base64;

#[cfg(test)]
mod base64_tests;

/* The rasteriser returns the capsule's Decoded; nothing else of the image
 * store is reached from svg, so only that shape is mirrored here. */
#[cfg(test)]
pub mod store {
    pub struct Decoded {
        pub w: u32,
        pub h: u32,
        pub px: alloc::vec::Vec<u32>,
    }
}

#[cfg(test)]
#[path = "../../../../capsule_browser/src/browser/image/svg/mod.rs"]
pub mod svg;

#[cfg(test)]
mod svg_tests;
