// NONOS Operating System (AGPL-3.0-or-later)
//! What the model drive answers IDENTIFY DEVICE with: QEMU's block for a
//! disk of the model's size, and a model name of its own.

use crate::constants::identify::{IDENTIFY_WORDS, W_MODEL};
use crate::identity_tests::qemu_block;

/// The model disk's size in sectors.
pub const CAPACITY: u64 = 1 << 20;
pub const MODEL: &[u8] = b"NONOS MODEL DISK";

pub fn block() -> [u16; IDENTIFY_WORDS] {
    let mut w = qemu_block(CAPACITY);
    // ATA strings: two characters a word, the first in the high byte.
    for (i, pair) in MODEL.chunks(2).enumerate() {
        w[W_MODEL + i] = u16::from(pair[0]) << 8 | u16::from(pair[1]);
    }
    w
}

/// The block's first `bytes` bytes, little-endian words, into host memory
/// at `to`.
pub(super) fn identify_into(to: u64, bytes: u64) {
    let raw: Vec<u8> = block().iter().flat_map(|w| w.to_le_bytes()).collect();
    let n = (bytes as usize).min(raw.len());
    // SAFETY: `to` is a buffer of the rig's that the PRD names, `bytes` long.
    unsafe { core::ptr::copy_nonoverlapping(raw.as_ptr(), to as *mut u8, n) };
}
