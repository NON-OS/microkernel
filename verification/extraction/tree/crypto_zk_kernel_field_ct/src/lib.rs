// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/crypto/zk_kernel/field/ct.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod crypto;

pub fn fieldelement_is_zero(this: crate::crypto::zk_kernel::field::types::FieldElement) -> bool {
    this.is_zero()
}

pub fn fieldelement_ct_is_zero(this: crate::crypto::zk_kernel::field::types::FieldElement) -> u8 {
    this.ct_is_zero()
}

