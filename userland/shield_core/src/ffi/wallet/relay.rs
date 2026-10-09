//! Handing a proved spend to the lander over Tor, from the hand-off folder of this wallet.

use super::Wallet;
use crate::error::WalletError;
use crate::net::pool::ACTIVE;
use crate::net::relay::{self, RelayState};

/// What became of a hand-off: queued under an id, or refused with a reason, which costs nothing.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RelayHandoff {
    pub id: Option<String>,
    pub refusal: Option<String>,
}

impl Wallet {
    /// Post the spend proved last to the lander, over its onion service, and start its timers.
    pub fn hand_to_relayer(&self) -> Result<RelayHandoff, WalletError> {
        let dir = self.handoff_dir();
        let prior = super::publish::read_record(&dir);
        let (record, refusal) = self.publish_from(&dir, prior.as_ref())?;
        Ok(match refusal {
            None => RelayHandoff { id: record.id, refusal: None },
            Some(reason) => RelayHandoff { id: None, refusal: Some(reason) },
        })
    }

    /// Where the hand-off `id` stands, asked of the lander that took it.
    pub fn relayer_state(&self, id: String) -> Result<RelayState, WalletError> {
        let at = super::publish::read_record(&self.handoff_dir()).map_or(0, |r| r.lander);
        let onion = ACTIVE.landers.get(at).copied().ok_or(WalletError::Unavailable)?;
        Ok(relay::state(self.tor()?.as_ref(), onion, &id)?)
    }

    /// Whether a lander is switched on in this build. With none, a spend is settled by its owner.
    pub fn has_lander(&self) -> bool {
        !ACTIVE.landers.is_empty()
    }
}
