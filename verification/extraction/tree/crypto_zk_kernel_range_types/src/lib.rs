// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/crypto/zk_kernel/range/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod crypto;

pub fn bitproof_verify_structure(this: crate::crypto::zk_kernel::range::types::BitProof) -> bool {
    this.verify_structure()
}

