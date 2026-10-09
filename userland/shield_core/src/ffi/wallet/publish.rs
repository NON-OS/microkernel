//! Publishing the spend proved last for anyone to land, through Tor and never direct. The route
//! today is the first of the pool's landers that answers. The record of what was published sits beside the hand-off, so the
//! timers of `follow_spend` outlast a restart.

use super::publish_view::{publication, Publication};
use super::Wallet;
use crate::error::{CustodyError, StoreError, WalletError};
use crate::net::pool::ACTIVE;
use crate::net::relay::{self, Handed};
use crate::wallet::publish::{Published, Route, RECORD};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

impl Wallet {
    /// Publish the spend proved last, once. A spend already published keeps its record and timers.
    pub fn publish_spend(&self) -> Result<Publication, WalletError> {
        self.with(|_| ())?;
        let dir = self.handoff_dir();
        if let Some(record) = read_record(&dir) {
            return Ok(publication(&record, None));
        }
        let (record, refusal) = self.publish_from(&dir, None)?;
        Ok(publication(&record, refusal))
    }
}

impl Wallet {
    pub(super) fn export_dir(&self) -> PathBuf {
        let root = self.paths().measurement.parent().map(Path::to_path_buf).unwrap_or_default();
        root.join("export")
    }

    pub(super) fn handoff_dir(&self) -> PathBuf {
        self.export_dir().join(crate::wallet::kept::HANDOFF)
    }

    /// Hand the spend in `dir` to the lander over Tor and write the record, counting on from
    /// `prior` when it is published again.
    pub(super) fn publish_from(
        &self,
        dir: &Path,
        prior: Option<&Published>,
    ) -> Result<(Published, Option<String>), WalletError> {
        if ACTIVE.landers.is_empty() {
            return Err(WalletError::Unavailable);
        }
        let body = crate::wallet::relay_body::body(dir)?;
        let tor = self.tor()?;
        let (lander, handed) = relay::hand_first(&tor, ACTIVE.landers, ACTIVE.address, &body)?;
        let (id, refusal) = match handed {
            Handed::Queued { id } => (Some(id), None),
            Handed::Refused { reason } => (prior.and_then(|p| p.id.clone()), Some(reason)),
        };
        let now = now()?;
        let first = prior.map_or(now, |p| p.first);
        let times = prior.map_or(1, |p| p.times.saturating_add(1));
        let record = Published { first, last: now, times, route: Route::Lander, id, lander };
        // A spend refused the first time was never published, so no timer starts for it.
        if refusal.is_none() || prior.is_some() {
            std::fs::write(dir.join(RECORD), record.to_text()).map_err(|_| StoreError::Io)?;
        }
        Ok((record, refusal))
    }
}

pub(super) fn read_record(dir: &Path) -> Option<Published> {
    Published::from_text(&std::fs::read_to_string(dir.join(RECORD)).ok()?)
}

pub(super) fn now() -> Result<u64, WalletError> {
    let since = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| CustodyError::Entropy)?;
    Ok(since.as_secs())
}
