//! Whether a spend has landed, from the `NullifierSpent` events every wallet reads alike.

use crate::error::{StoreError, WalletError};
use crate::net::rpc::RawLog;
use crate::prover::launch::publics::{pool_words, LIMBS};
use crate::wallet::relay_body::{limbs, read};

/// What the history shows of the two nullifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Seen {
    Neither,
    /// Both, in one settlement, with its transaction when the server named it.
    Landed(Option<[u8; 32]>),
    /// One or both, not together: another proof spent a note this spend would have.
    Elsewhere,
}

/// The two nullifiers of a spend, as the pool emits them, from its public limbs.
pub fn nullifiers(limbs: &[u64; LIMBS]) -> [[u8; 32]; 2] {
    let words = pool_words(limbs);
    [words[2], words[3]]
}

/// The two nullifiers of the spend whose hand-off is in `dir`.
pub fn handoff_nullifiers(dir: &std::path::Path) -> Result<[[u8; 32]; 2], WalletError> {
    let text = String::from_utf8(read(dir, "spend.proof.publics.json")?)
        .map_err(|_| StoreError::RowShape)?;
    let limbs: [u64; LIMBS] =
        limbs(&text).and_then(|l| l.try_into().ok()).ok_or(StoreError::RowShape)?;
    Ok(nullifiers(&limbs))
}

pub fn landed(spent: &[RawLog], nullifiers: &[[u8; 32]; 2]) -> Seen {
    let find = |nf: &[u8; 32]| spent.iter().find(|log| log.topics.get(1) == Some(nf));
    match (find(&nullifiers[0]), find(&nullifiers[1])) {
        (None, None) => Seen::Neither,
        (Some(a), Some(b)) if a.block == b.block && a.tx == b.tx => Seen::Landed(a.tx),
        _ => Seen::Elsewhere,
    }
}
