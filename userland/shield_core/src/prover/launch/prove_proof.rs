//! What a proved spend hands on to settlement.

use super::super::publics::{LIMBS, WORDS};
use crate::notes::NotePlaintext;

/// A proved spend and everything settlement needs from it.
pub struct LaunchProof {
    pub bytes: Vec<u8>,
    pub publics: [u64; LIMBS],
    pub words: [[u8; 32]; WORDS],
    /// The two notes created, payee first, with the blindings drawn for them.
    pub outputs: [NotePlaintext; 2],
    pub grind_hashes: u64,
    pub weakened: Vec<&'static str>,
    /// The periodic cache, when this proof built it, for the caller to keep.
    pub new_cache: Option<Vec<u8>>,
}
