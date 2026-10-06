//! Writing the history down as it happens (`store::activity`): each event is a row of the
//! account's own store, sealed with its notes.

use crate::error::StoreError;
use crate::notes::NotePlaintext;
use crate::store::activity::{settled, Activity};
use crate::store::Row;
use crate::wallet::Session;
use std::time::{SystemTime, UNIX_EPOCH};

/// Seconds since 1970, or zero on a clock that will not say: the entry is kept all the same.
pub fn clock() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// A note's commitment as the store keys it.
pub fn cm_of(plain: &NotePlaintext) -> [u64; 4] {
    crate::notes::commitment(&crate::prover::pool_hasher(), &plain.note()).map(|f| f.value())
}

/// Record `a` for the active account.
pub fn note(s: &mut Session, a: Activity) -> Result<(), StoreError> {
    s.record(Row::Activity(a))
}

/// Record that the spend `tag` landed, or was taken back, once.
pub fn once(s: &mut Session, a: Activity) -> Result<(), StoreError> {
    if settled(s.state().activity(), &a.tag) {
        return Ok(());
    }
    note(s, a)
}
