//! The yes of the owner to a split deposit: each deposit signed and sent in order on its own nonce,
//! its note stored before it leaves, and the first that fails stops the rest.

use super::super::account_view::VALID_FOR;
use super::super::Wallet;
use crate::error::{CustodyError, NetError, WalletError};
use crate::evm::{self, Network};
use crate::ffi::account_types::PublicSent;
use crate::ffi::evm::to_hex;
use crate::store::Row;

/// The deposits sent in order, and why the rest were not, when one failed.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShieldSplitSent {
    pub sent: Vec<PublicSent>,
    pub stopped: Option<String>,
}

impl Wallet {
    /// Sign and send the deposits held under `id`.
    pub fn confirm_shield_split(&self, id: u64) -> Result<ShieldSplitSent, WalletError> {
        let batch = {
            let mut slot = self.batch.lock().map_err(|_| NetError::Transport)?;
            if slot.as_ref().is_none_or(|b| b.id != id) {
                return Err(WalletError::ReviewExpired);
            }
            slot.take().ok_or(WalletError::ReviewExpired)?
        };
        let (key, from) = self.with(|s| (s.evm().signing_key(), s.evm().address()))?;
        if batch.made.elapsed() >= VALID_FOR || from != batch.from {
            return Err(WalletError::ReviewExpired);
        }
        let key = key.ok_or(WalletError::Custody { source: CustodyError::Locked })?;
        let tor = self.tor()?;
        let mut sent = Vec::with_capacity(batch.steps.len());
        for (tx, note) in batch.steps {
            // A lost blinding is money lost, so the note is kept before its deposit leaves.
            self.with_mut(|s| Ok(s.record(Row::Deposit(note))?))?;
            let hash = match evm::broadcast(&tor, Network::Sepolia, &tx, &key) {
                Ok(hash) => hash,
                Err(e) => return Ok(ShieldSplitSent { sent, stopped: Some(e.to_string()) }),
            };
            let _ =
                evm::nonce::record(&self.paths().account, Network::Sepolia, &from, tx.nonce, &hash);
            let link = format!("{}{}", Network::Sepolia.chain().explorer, to_hex(&hash));
            sent.push(PublicSent { hash: to_hex(&hash), link });
        }
        Ok(ShieldSplitSent { sent, stopped: None })
    }
}
