// NONOS Operating System (AGPL-3.0-or-later)
//! Whether text is mapped guarded: the kernel asks FEAT_BTI and how it was
//! built; a test says.

use super::PageAttributes;
use core::sync::atomic::{AtomicBool, Ordering};

pub static GUARDED: AtomicBool = AtomicBool::new(false);

pub(super) fn code() -> PageAttributes {
    PageAttributes::kernel_code().with_guard(GUARDED.load(Ordering::Relaxed))
}
