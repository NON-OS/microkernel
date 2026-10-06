//! The owner's yes to a held review: a send, a swap or a deposit. The review must be current and
//! worked out for the account now unlocked, and a deposit's note is stored before it leaves.

use super::account_view::VALID_FOR;
use super::Wallet;
use crate::error::{NetError, WalletError};
use crate::evm;
use crate::ffi::account_types::PublicSent;
use crate::ffi::evm::to_hex;

impl Wallet {
    /// Sign and send the transaction held under `id`.
    pub fn confirm_public_send(&self, id: u64) -> Result<PublicSent, WalletError> {
        let pending = {
            let mut slot = self.pending.lock().map_err(|_| NetError::Transport)?;
            if slot.as_ref().is_none_or(|p| p.id != id) {
                return Err(WalletError::ReviewExpired);
            }
            slot.take().ok_or(WalletError::ReviewExpired)?
        };
        if pending.made.elapsed() >= VALID_FOR {
            return Err(WalletError::ReviewExpired);
        }
        let (key, from) = self.with(|s| (s.evm().signing_key(), s.evm().address()))?;
        // Else the nonce, balance and sender shown belong to another account.
        if from != pending.from {
            return Err(WalletError::ReviewExpired);
        }
        let key = key.ok_or(WalletError::Custody { source: crate::error::CustodyError::Locked })?;
        // A deposit's note is kept before the deposit leaves: a lost blinding is money lost.
        if let Some(note) = pending.note.clone() {
            self.with_mut(|s| Ok(s.record(crate::store::Row::Deposit(note))?))?;
        }
        let hash = evm::broadcast(self.tor()?.as_ref(), pending.network, &pending.tx, &key)?;
        // A settlement the owner sent: its spend is never handed to a lander again.
        let settles = self.settles.lock().ok().and_then(|mut s| s.take());
        if let Some((_, dir)) = settles.filter(|(id, _)| *id == pending.id) {
            let _ = crate::wallet::kept::mark_settling(&dir);
            self.note_settled(&dir, hash);
        }
        if let Some(note) = &pending.note {
            self.note_deposit(note, hash);
        }
        // Sent either way. A failed write only means the next review cannot ask about it.
        let dir = &self.paths().account;
        let _ = evm::nonce::record(dir, pending.network, &from, pending.tx.nonce, &hash);
        Ok(PublicSent {
            hash: to_hex(&hash),
            link: format!("{}{}", pending.network.chain().explorer, to_hex(&hash)),
        })
    }
}

impl Wallet {
    /// A deposit gone out, into the history. Sent either way: a failed write loses the entry,
    /// never the deposit, whose note is already kept.
    fn note_deposit(&self, note: &crate::notes::NotePlaintext, hash: [u8; 32]) {
        use crate::store::activity::{Activity, DEPOSIT_SENT};
        let mut a = Activity::new(DEPOSIT_SENT, crate::wallet::history::clock());
        a.asset_id = note.asset_id;
        a.value = note.value;
        a.tx = hash;
        a.cm = crate::wallet::history::cm_of(note);
        let _ = self.with_mut(|s| Ok(crate::wallet::history::note(s, a)?));
    }

    /// The owner's settlement of the spend in `dir`, into the history.
    fn note_settled(&self, dir: &std::path::Path, hash: [u8; 32]) {
        use crate::store::activity::{Activity, SETTLED_SELF};
        let Ok([tag, _]) = crate::wallet::publish::handoff_nullifiers(dir) else { return };
        let mut a = Activity::new(SETTLED_SELF, crate::wallet::history::clock());
        a.tag = tag;
        a.tx = hash;
        let _ = self.with_mut(|s| Ok(crate::wallet::history::note(s, a)?));
    }
}
