// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/crypto/zk_kernel/plonk/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/crypto/zk_kernel/plonk/types.rs"]
pub mod types;

pub fn plonkevaluations_new() -> types::PlonkEvaluations {
    types::PlonkEvaluations::new()
}

pub fn plonkcircuit_new() -> types::PlonkCircuit {
    types::PlonkCircuit::new()
}

