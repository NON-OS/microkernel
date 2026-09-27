// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/crypto/asymmetric/ed25519/field/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/crypto/asymmetric/ed25519/field/types.rs"]
pub mod types;

pub fn fe_zero() -> types::Fe {
    types::Fe::zero()
}

pub fn fe_one() -> types::Fe {
    types::Fe::one()
}

