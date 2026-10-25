// NONOS Operating System (AGPL-3.0-or-later)
//! The image where the kernel links it: text from 0x4008_0000, read-only data
//! to 0x4028_0000, writable data from the next 2 MiB.

pub const TEXT: (u64, u64) = (0x4008_0000, 0x4018_0000);
pub const IMAGE: (u64, u64) = (0x4008_0000, 0x4028_0000);

pub(super) fn image_bounds() -> (u64, u64) {
    IMAGE
}

pub(super) fn text_bounds() -> (u64, u64) {
    TEXT
}
