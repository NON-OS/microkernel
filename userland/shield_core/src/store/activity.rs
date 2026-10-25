//! What this wallet did with the pool, as rows of the store: deposits sent and taken in, spends
//! proved, landed, settled by their owner or taken back, and payments received. Kept beside the
//! notes, sealed the same way, so the history outlives a restart and goes wherever the store
//! goes. A spend is named by its first nullifier, which its follow reads too, and carries its
//! change note's commitment, so the change a sync finds is never taken for a payment received.

#[path = "activity_entry.rs"]
mod entry;
#[path = "activity_fold.rs"]
mod fold;
#[path = "activity_kind.rs"]
mod kind;

pub use entry::{Entry, Stage};
pub use fold::history;
pub use kind::{Activity, DEPOSIT_IN, DEPOSIT_SENT, LANDED, RECEIVED, SENT, SETTLED_SELF};
pub use kind::{TAKEN_BACK, WITHDRAWN};

/// Whether the rows already say the spend `tag` landed or was taken back.
pub fn settled(rows: &[Activity], tag: &[u8; 32]) -> bool {
    rows.iter().any(|a| matches!(a.kind, LANDED | TAKEN_BACK) && a.tag == *tag)
}

/// Whether `cm` is the change of one of this wallet's own spends.
pub fn is_change(rows: &[Activity], cm: &[u64; 4]) -> bool {
    rows.iter().any(|a| matches!(a.kind, SENT | WITHDRAWN) && a.cm == *cm)
}

/// 32 bytes as the four little endian words a row carries, and back.
pub(crate) fn to_quad(b: &[u8; 32]) -> [u64; 4] {
    let mut q = [0u64; 4];
    for (w, chunk) in q.iter_mut().zip(b.chunks_exact(8)) {
        let mut x = [0u8; 8];
        x.copy_from_slice(chunk);
        *w = u64::from_le_bytes(x);
    }
    q
}

pub(crate) fn from_quad(q: &[u64; 4]) -> [u8; 32] {
    let mut b = [0u8; 32];
    for (chunk, w) in b.chunks_exact_mut(8).zip(q) {
        chunk.copy_from_slice(&w.to_le_bytes());
    }
    b
}

#[cfg(test)]
#[path = "activity_test.rs"]
mod activity_test;
