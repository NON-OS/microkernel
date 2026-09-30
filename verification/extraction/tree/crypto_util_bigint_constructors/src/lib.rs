// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/crypto/util/bigint/constructors.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod crypto;

pub fn biguint_zero() -> crate::crypto::util::bigint::types::BigUint {
    crate::crypto::util::bigint::types::BigUint::zero()
}

pub fn biguint_one() -> crate::crypto::util::bigint::types::BigUint {
    crate::crypto::util::bigint::types::BigUint::one()
}

pub fn biguint_new() -> crate::crypto::util::bigint::types::BigUint {
    crate::crypto::util::bigint::types::BigUint::new()
}

pub fn biguint_from_u64(val: u64) -> crate::crypto::util::bigint::types::BigUint {
    crate::crypto::util::bigint::types::BigUint::from_u64(val)
}

