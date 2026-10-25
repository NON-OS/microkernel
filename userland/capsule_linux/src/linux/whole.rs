// NONOS Operating System (AGPL-3.0-or-later)

//! A file taken whole or not at all. A read capped at the most a caller
//! takes cuts a longer file silently, and a cut keymap, manifest or path
//! reads as another one; so the read asks for one byte more, and a file
//! that fills it is refused.

use alloc::vec::Vec;

/// What to ask a read for when at most `max` bytes are taken.
pub const fn ask(max: u32) -> u32 {
    max.saturating_add(1)
}

/// `bytes`, read with [`ask`], when they are the whole file: no more than
/// `max`.
pub fn whole(bytes: Vec<u8>, max: u32) -> Option<Vec<u8>> {
    (bytes.len() as u64 <= u64::from(max)).then_some(bytes)
}
