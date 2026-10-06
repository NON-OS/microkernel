//! The pool's note tree, rebuilt here from the leaves the chain published.
//!
//! The prover rebuilds it too and refuses a request whose leaves do not reach
//! the root it names. Rebuilding it here first lets the wallet check a scanned
//! history against the pool's own roots before minutes are spent proving.

use nonos_stark::air::RATE;
use nonos_stark::field::Fp;
use stark_proofs::crypto::stark::air::Poseidon;
use stark_proofs::shield::member::{PoolTree, TREE_DEPTH};
use stark_proofs::shield::note::POOL_LOG_ROUNDS;

/// The root over `leaves`, in leaf order, as the pool computes it.
pub fn pool_root(leaves: &[[u64; RATE]]) -> [u64; RATE] {
    let mut tree =
        PoolTree::with_depth(Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]), TREE_DEPTH);
    for leaf in leaves {
        tree.insert(leaf.map(Fp::from_u64));
    }
    tree.root().map(|f| f.to_u64())
}

/// A digest as the 256 bit hex word the pool and the prover read: limb 0 is
/// the lowest 64 bits.
pub fn word_hex(limbs: &[u64; RATE]) -> String {
    format!("0x{:016x}{:016x}{:016x}{:016x}", limbs[3], limbs[2], limbs[1], limbs[0])
}
