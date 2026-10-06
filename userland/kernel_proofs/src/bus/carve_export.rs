// NONOS Operating System (AGPL-3.0-or-later)

//! `carve` is `pub(crate)` in the kernel, where only the window allocator
//! calls it. This hands the same function to the `pci_window` harnesses.

/// The kernel's `carve`, called through unchanged.
pub fn carve(cursor: u64, limit: u64, size: u64) -> Option<(u64, u64)> {
    super::carve::carve(cursor, limit, size)
}
