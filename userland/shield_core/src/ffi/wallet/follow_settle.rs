//! When the owner may settle a spend from the public account.

use super::super::publish::{now, read_record};
use super::super::Wallet;
use crate::error::WalletError;
use crate::net::pool::ACTIVE;
use crate::wallet::kept::settling;
use crate::wallet::publish::SELF_SETTLE_AFTER;
use std::path::Path;

impl Wallet {
    /// Whether the owner may settle the spend proved last now: with no lander, with nothing
    /// published, or 30 minutes after the first publication.
    pub(in crate::ffi::wallet) fn self_settle_open(&self) -> Result<bool, WalletError> {
        self.self_settle_open_in(&self.handoff_dir())
    }

    /// The same for the hand-off in `dir`. Never again once its owner sent the settlement.
    pub(in crate::ffi::wallet) fn self_settle_open_in(
        &self,
        dir: &Path,
    ) -> Result<bool, WalletError> {
        if settling(dir) {
            return Ok(false);
        }
        let Some(record) = read_record(dir) else { return Ok(true) };
        Ok(ACTIVE.landers.is_empty() || now()? >= record.first.saturating_add(SELF_SETTLE_AFTER))
    }
}
