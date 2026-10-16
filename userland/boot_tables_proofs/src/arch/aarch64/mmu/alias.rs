// NONOS Operating System (AGPL-3.0-or-later)
//! The image's two addresses, for the proofs that walk both.

use super::geometry::KERNEL_SPACE_START;
use super::image_bounds::TEXT;

/// Whether `va` is the image's text, one to one or at its high alias: the
/// kernel runs from the first until it moves, then from the second.
pub fn is_text(va: u64) -> bool {
    (TEXT.0..TEXT.1).contains(&va)
        || (TEXT.0..TEXT.1).contains(&va.wrapping_sub(KERNEL_SPACE_START))
}

/// Text pages over both addresses.
pub fn text_pages() -> u64 {
    2 * (TEXT.1 - TEXT.0) / 4096
}

/// Whether the build gave every image block and boot device its alias.
pub fn high_mapped() -> bool {
    super::kernel_va::high_mapped()
}
