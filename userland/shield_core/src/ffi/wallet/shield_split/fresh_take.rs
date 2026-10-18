//! A SHIELD_FROM_FRESH review taken to confirm (v1.4.56): its deposits, or
//! else its approval, once, and only while VALID_FOR holds.

use super::super::account_view::VALID_FOR;
use super::super::Wallet;
use crate::error::{NetError, WalletError};
use crate::evm::tx::Eip1559;
use crate::notes::NotePlaintext;

pub(super) type Steps = Vec<(Eip1559, Option<NotePlaintext>)>;

impl Wallet {
    /// The deposits held under `id`, or else its approval, taken once.
    pub(super) fn take_fresh(&self, id: u64) -> Result<([u8; 20], Steps), WalletError> {
        let mut batch = self.batch.lock().map_err(|_| NetError::Transport)?;
        if let Some(b) = batch.take_if(|b| b.id == id) {
            let fresh = b.made.elapsed() < VALID_FOR;
            let steps = b.steps.into_iter().map(|(tx, note)| (tx, Some(note))).collect();
            return fresh.then_some((b.from, steps)).ok_or(WalletError::ReviewExpired);
        }
        let mut pending = self.pending.lock().map_err(|_| NetError::Transport)?;
        let p = pending.take_if(|p| p.id == id).ok_or(WalletError::ReviewExpired)?;
        let fresh = p.made.elapsed() < VALID_FOR;
        fresh.then_some((p.from, vec![(p.tx, p.note)])).ok_or(WalletError::ReviewExpired)
    }
}
