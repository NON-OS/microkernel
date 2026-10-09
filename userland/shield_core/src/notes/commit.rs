//! The note commitment, recomputed and never taken from a server.
//! It is the nested commitment the pool computes and the circuit proves. Changing the byte order
//! of the associated data form breaks every stored note.

use nonos_stark::air::{Poseidon, RATE};
use nonos_stark::field::Fp;
use stark_proofs::shield::note::{owner_commit, quads, Note};

/// The note commitment. A commitment the wallet did not derive is one it cannot spend.
pub fn commitment(h: &Poseidon, note: &Note) -> [Fp; RATE] {
    // The pool's `note_parts`: owner digest over the secret quads, then the public quad and it.
    let q = quads(&note.limbs());
    let owner = h.compress(&q[0], &q[1]);
    h.compress(&q[2], &owner)
}

/// The commitment as little endian bytes, the AAD that binds a ciphertext to its leaf.
pub fn commitment_bytes(cm: &[Fp; RATE]) -> [u8; RATE * 8] {
    let mut out = [0u8; RATE * 8];
    for (slot, element) in out.chunks_exact_mut(8).zip(cm.iter()) {
        slot.copy_from_slice(&element.value().to_le_bytes());
    }
    out
}

/// The owner digest the pool's `absorb` takes, `compress(spend_pk, blinding)`.
/// The pool computes the outer compression from the value it escrows, so a deposit cannot lie.
pub fn owner_commit_wire(note: &Note) -> [u8; 32] {
    let o = owner_commit(note);
    crate::notes::wire_digest(&[o[0].value(), o[1].value(), o[2].value(), o[3].value()])
}
