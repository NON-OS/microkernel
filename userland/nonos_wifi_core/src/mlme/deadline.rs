// NONOS Operating System (AGPL-3.0-or-later)

//! Each step of a join given its own time, so a step the access point never
//! answers ends the join, named, before the driver's budget does. The times
//! are mac80211's and hostapd's, with room for the drivers' resends.

use super::{Mlme, MlmeFailure, MlmeState};

/// Open System authentication: mac80211's three tries at 200 ms
/// (IEEE80211_AUTH_TIMEOUT, IEEE80211_AUTH_MAX_TRIES), widened so a driver
/// resending every 300 ms still sends three.
pub const AUTH_MS: u64 = 1_000;
/// SAE, commit and confirm: mac80211's three tries at 2 s
/// (IEEE80211_AUTH_TIMEOUT_SAE).
pub const SAE_MS: u64 = 6_000;
/// Association: as Open System authentication (IEEE80211_ASSOC_TIMEOUT).
pub const ASSOC_MS: u64 = 1_000;
/// The four-way handshake, from association: hostapd sends each message
/// four times a second apart (dot11RSNAConfigPairwiseUpdateTimeOut and
/// Count), and the station waits one second more.
pub const FOURWAY_MS: u64 = 5_000;

impl Mlme {
    /// When the current step must end by: its allowance, from when this clock
    /// first saw it. `None` in a step with none (watching beacons, connected,
    /// failed).
    pub fn deadline(&mut self, now_ms: u64) -> Option<u64> {
        let allowance = match self.state {
            MlmeState::Authenticating if self.sae.is_some() => SAE_MS,
            MlmeState::Authenticating => AUTH_MS,
            MlmeState::Associating => ASSOC_MS,
            MlmeState::FourWay => FOURWAY_MS,
            _ => return None,
        };
        let since = match self.step_since {
            Some((step, since)) if step == self.state => since,
            _ => self.step_since.insert((self.state, now_ms)).1,
        };
        Some(since.saturating_add(allowance))
    }

    /// The join failed because a step ran out of its time, not on a refusal.
    pub fn expired(&self) -> bool {
        self.expired
    }

    /// Past the step's deadline the join fails: with why its last SAE frame
    /// was discarded, where one was, or else naming the step. True once it
    /// has.
    pub fn expire(&mut self, now_ms: u64) -> bool {
        match self.deadline(now_ms) {
            Some(end) if now_ms >= end => {
                let why = self.failure().unwrap_or(MlmeFailure::Timeout(self.state));
                self.fail(why);
                self.expired = true;
                true
            }
            _ => false,
        }
    }
}
