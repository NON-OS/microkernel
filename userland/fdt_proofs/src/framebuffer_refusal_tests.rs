// NONOS Operating System (AGPL-3.0-or-later)
//! The `simple-framebuffer` nodes the finder must refuse rather than draw
//! into: a format it cannot draw, and rows that do not fit their memory.

use crate::arch::fdt::find::framebuffer::Refused;
use crate::framebuffer_tree::{found, tree};

#[test]
fn a_format_the_kernel_cannot_draw_is_refused() {
    for f in ["r5g6b5", "x1r5g5b5", "a2r10g10b10", "x8r8g8b8 ", "", "A8R8G8B8"] {
        assert_eq!(found(&tree("", None, f, 7680, "okay")), Err(Refused::UnknownFormat), "{f:?}");
    }
}

#[test]
fn geometry_that_runs_past_its_memory_is_refused() {
    // A row narrower than 1920 four-byte pixels.
    assert_eq!(found(&tree("", None, "a8r8g8b8", 7676, "okay")), Err(Refused::Geometry));
    // Rows that need more than `reg` covers.
    assert_eq!(found(&tree("", None, "a8r8g8b8", 8192, "okay")), Err(Refused::Geometry));
}
