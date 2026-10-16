//! Following every spend still on its way, read over Tor as every wallet reads the pool: the
//! spend published last and each one kept before it. Each is published again after 15 minutes
//! and offered to its owner after 30, unless its owner already sent the settlement, and an
//! earlier one is forgotten once it lands or a note of it is spent another way. A lock stops
//! either read at its next request.

#[path = "follow_end.rs"]
mod end;
#[path = "follow_kept.rs"]
mod kept;
#[path = "follow_settle.rs"]
mod settle;

use super::publish::read_record;
use super::publish_view::{unpublished, KeptFollow, SpendFollow};
use super::Wallet;
use crate::error::WalletError;
use crate::net::rpc::progress::Stop;
use crate::wallet::kept::{earlier, forget};
use crate::wallet::publish::{handoff_nullifiers, landed};

impl Wallet {
    /// One look at the spend published last. Asked at most once a minute, as a scan is.
    pub fn follow_spend(&self) -> Result<SpendFollow, WalletError> {
        let stop = Stop::watching(&self.stops);
        self.with(|_| ())?;
        let dir = self.handoff_dir();
        let record = read_record(&dir).ok_or(WalletError::Unavailable)?;
        let history = self.pool_history(self.tor()?.as_ref(), stop)?;
        let pair = handoff_nullifiers(&dir)?;
        self.follow_in(&dir, &record, landed(&history.nullifiers, &pair))
    }

    /// One look at every spend kept before the last, oldest first, on one read of the pool. One
    /// that landed, or whose notes went another way, is reported once and forgotten.
    pub fn follow_earlier(&self) -> Result<Vec<KeptFollow>, WalletError> {
        let stop = Stop::watching(&self.stops);
        self.with(|_| ())?;
        let kept = earlier(&self.export_dir());
        if kept.is_empty() {
            return Ok(Vec::new());
        }
        let history = self.pool_history(self.tor()?.as_ref(), stop)?;
        let mut out = Vec::with_capacity(kept.len());
        for (id, dir) in kept {
            let follow = match self.follow_kept(&dir, &history.nullifiers) {
                Ok(f) => f,
                Err(e) => SpendFollow { refusal: Some(e.to_string()), ..unpublished("not read") },
            };
            if matches!(follow.state.as_str(), "landed" | "spent elsewhere") {
                forget(&dir);
            }
            out.push(KeptFollow { id, follow });
        }
        Ok(out)
    }
}
