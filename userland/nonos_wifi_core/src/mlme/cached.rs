// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Joining on a cached PMKSA (IEEE Std 802.11-2020, 12.6.10.3). When the
//! network's AKM is SAE and a PMKSA for this very AP was offered, the join
//! skips SAE: Open System authentication, then an Association Request whose
//! RSNE lists the PMKID, then the four-way handshake under the cached PMK;
//! an AP that has forgotten it refuses, and the caller runs SAE.

use alloc::vec::Vec;

use super::failure::MlmeFailure;
use super::state::{Mlme, MlmeOutput, MlmeState};
use crate::dot11::mgmt::auth_open;
use crate::pmksa::Pmksa;
use crate::rsn::build::{station_rsne, station_rsne_pmkid};
use crate::rsn::Selection;
use crate::wpa::akm::Akm;

impl Mlme {
    /// Offer `p`; used only if the network runs SAE and the AP is `p.bssid`.
    pub fn offer_pmksa(&mut self, p: Pmksa) {
        self.cached = Some(p);
    }

    /// Whether this join ran on an offered PMKSA instead of SAE.
    pub fn used_pmksa(&self) -> bool {
        self.cache_used
    }

    /// The PMKSA a connected SAE join holds, for the caller to cache.
    pub fn pmksa(&self) -> Option<Pmksa> {
        let sae = self.state == MlmeState::Connected && self.akm() == Some(Akm::Sae);
        self.pmkid.filter(|_| sae).map(|pmkid| Pmksa { bssid: self.bssid, pmk: self.pmk, pmkid })
    }

    pub(super) fn start_sae_or_cached(&mut self, h2e: bool) -> MlmeOutput {
        // Taken: the offer is used once, and wiped as `p` is dropped.
        let Some(p) = self.cached.take_if(|p| p.bssid == self.bssid) else {
            return self.start_sae(h2e);
        };
        (self.pmk, self.pmkid, self.cache_used) = (p.pmk, Some(p.pmkid), true);
        self.forget_secret();
        let mut out = [0u8; 64];
        match auth_open(&mut out, self.our_mac, self.bssid, self.next_seq()) {
            Some(n) => {
                self.state = MlmeState::Authenticating;
                MlmeOutput::send(out[..n].to_vec())
            }
            None => self.fail(MlmeFailure::AuthRejected(0)),
        }
    }

    /// The RSNE this station sends, in the request and again in message 2:
    /// with the PMKID only when the join runs on a cached PMKSA.
    pub(super) fn own_rsne(&self, sel: &Selection) -> Vec<u8> {
        match self.pmkid.filter(|_| self.cache_used) {
            Some(id) => station_rsne_pmkid(sel, &id).to_vec(),
            None => station_rsne(sel).to_vec(),
        }
    }
}
