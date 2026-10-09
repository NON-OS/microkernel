//! One pool output, as the sequencer serves it.
//!
//! Every record is the same size, so the length of a reply says how many
//! outputs it carries and nothing about which of them are ours.

use crate::notes::CIPHER_LEN;

/// One pool output as the sequencer serves it: where it sits in the tree, the
/// commitment the tree holds, and the note ciphertext that travelled with it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PoolOutput {
    pub leaf_index: u64,
    pub cm: [u64; 4],
    pub cipher: [u8; CIPHER_LEN],
}
