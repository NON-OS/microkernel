//! Following every spend still on its way, read over Tor as every wallet reads the pool: the
//! spend published last and each one kept before it. Each is published again after 15 minutes
//! and offered to its owner after 30, unless its owner already sent the settlement, and an
//! earlier one is forgotten once it lands or a note of it is spent another way. A lock stops
//! either read at its next request.

use super::publish::{now, read_record};
use super::publish_view::{follow, unpublished, KeptFollow, SpendFollow};
use super::Wallet;
use crate::error::WalletError;
use crate::net::pool::ACTIVE;
use crate::net::rpc::progress::Stop;
use crate::net::rpc::RawLog;
use crate::wallet::kept::{earlier, forget, settling};
use crate::wallet::publish::{
    handoff_nullifiers, landed, next, next_settling, Next, Seen, SELF_SETTLE_AFTER,
};
use std::path::Path;

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

impl Wallet {
    fn follow_kept(&self, dir: &Path, spent: &[RawLog]) -> Result<SpendFollow, WalletError> {
        let seen = landed(spent, &handoff_nullifiers(dir)?);
        if let Some(record) = read_record(dir) {
            return self.follow_in(dir, &record, seen);
        }
        // Never taken by a lander: offered to a lander again, unless it is done or its owner
        // sent the settlement, and offered to its owner meanwhile.
        let now = now()?;
        self.note_end(dir, seen);
        match seen {
            Seen::Landed(_) | Seen::Elsewhere => {
                let blank = crate::wallet::publish::Published {
                    first: now,
                    last: now,
                    times: 0,
                    route: crate::wallet::publish::Route::Lander,
                    id: None,
                    lander: 0,
                };
                let (step, _) = next(now, &blank, seen);
                Ok(follow(&blank, now, step, false))
            }
            Seen::Neither if settling(dir) => Ok(unpublished("waiting")),
            Seen::Neither => match self.publish_from(dir, None) {
                Ok((record, None)) => Ok(follow(&record, now, Next::Wait, false)),
                Ok((_, Some(why))) => {
                    Ok(SpendFollow { refusal: Some(why), ..unpublished("unpublished") })
                }
                Err(WalletError::Unavailable) => Ok(unpublished("unpublished")),
                Err(e) => Err(e),
            },
        }
    }

    fn follow_in(
        &self,
        dir: &Path,
        record: &crate::wallet::publish::Published,
        seen: Seen,
    ) -> Result<SpendFollow, WalletError> {
        let now = now()?;
        self.note_end(dir, seen);
        let (step, offered) =
            if settling(dir) { next_settling(now, record, seen) } else { next(now, record, seen) };
        if step != Next::Republish {
            return Ok(follow(record, now, step, offered));
        }
        let (again, refusal) = self.publish_from(dir, Some(record))?;
        Ok(SpendFollow { refusal, ..follow(&again, now, step, offered) })
    }
}

impl Wallet {
    /// Whether the owner may settle the spend proved last now: with no lander, with nothing
    /// published, or 30 minutes after the first publication.
    pub(super) fn self_settle_open(&self) -> Result<bool, WalletError> {
        self.self_settle_open_in(&self.handoff_dir())
    }

    /// The same for the hand-off in `dir`. Never again once its owner sent the settlement.
    pub(super) fn self_settle_open_in(&self, dir: &Path) -> Result<bool, WalletError> {
        if settling(dir) {
            return Ok(false);
        }
        let Some(record) = read_record(dir) else { return Ok(true) };
        Ok(ACTIVE.landers.is_empty() || now()? >= record.first.saturating_add(SELF_SETTLE_AFTER))
    }
}

impl Wallet {
    /// A spend seen at its end, into the history once: landed, or, when another proof spent a
    /// note of it, never to land. A settlement its owner sent lands it even when it shows as
    /// spent another way.
    fn note_end(&self, dir: &Path, seen: Seen) {
        use crate::store::activity::{Activity, LANDED, TAKEN_BACK};
        let (kind, tx) = match seen {
            Seen::Landed(tx) => (LANDED, tx.unwrap_or([0; 32])),
            Seen::Elsewhere if settling(dir) => (LANDED, [0; 32]),
            Seen::Elsewhere => (TAKEN_BACK, [0; 32]),
            Seen::Neither => return,
        };
        let Ok([tag, _]) = handoff_nullifiers(dir) else { return };
        let mut a = Activity::new(kind, crate::wallet::history::clock());
        a.tag = tag;
        a.tx = tx;
        let _ = self.with_mut(|s| Ok(crate::wallet::history::once(s, a)?));
    }
}
