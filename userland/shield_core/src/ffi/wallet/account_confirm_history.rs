//! A confirmed deposit or owner's settlement, into the history.

use super::super::Wallet;

impl Wallet {
    /// A deposit gone out, into the history. Sent either way: a failed write loses the entry,
    /// never the deposit, whose note is already kept.
    pub(super) fn note_deposit(&self, note: &crate::notes::NotePlaintext, hash: [u8; 32]) {
        use crate::store::activity::{Activity, DEPOSIT_SENT};
        let mut a = Activity::new(DEPOSIT_SENT, crate::wallet::history::clock());
        a.asset_id = note.asset_id;
        a.value = note.value;
        a.tx = hash;
        a.cm = crate::wallet::history::cm_of(note);
        let _ = self.with_mut(|s| Ok(crate::wallet::history::note(s, a)?));
    }

    /// The owner's settlement of the spend in `dir`, into the history.
    pub(super) fn note_settled(&self, dir: &std::path::Path, hash: [u8; 32]) {
        use crate::store::activity::{Activity, SETTLED_SELF};
        let Ok([tag, _]) = crate::wallet::publish::handoff_nullifiers(dir) else { return };
        let mut a = Activity::new(SETTLED_SELF, crate::wallet::history::clock());
        a.tag = tag;
        a.tx = hash;
        let _ = self.with_mut(|s| Ok(crate::wallet::history::note(s, a)?));
    }
}
