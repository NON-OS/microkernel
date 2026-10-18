//! Following one spend from its hand-off: published again, offered to its owner, or done.

use super::super::publish::{now, read_record};
use super::super::publish_view::{follow, unpublished, SpendFollow};
use super::super::Wallet;
use crate::error::WalletError;
use crate::net::rpc::RawLog;
use crate::wallet::kept::settling;
use crate::wallet::publish::{handoff_nullifiers, landed, next, next_settling, Next, Seen};
use std::path::Path;

impl Wallet {
    pub(super) fn follow_kept(
        &self,
        dir: &Path,
        spent: &[RawLog],
    ) -> Result<SpendFollow, WalletError> {
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

    pub(super) fn follow_in(
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
