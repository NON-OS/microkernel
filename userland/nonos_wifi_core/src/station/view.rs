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

//! How the link starts and ends, and what the driver may ask it.

use super::pmf_state::PmfState;
use super::replay::Replay;
use super::watch::Watch;
use super::{Ccmp, LinkStation};
use crate::dot11::header::MacAddr;
use crate::wpa::supplicant::State;

impl LinkStation {
    /// Record association to `bssid` with the given CCMP mode, resetting the
    /// sequence and packet-number counters for the new session.
    pub fn associate(&mut self, bssid: MacAddr, ccmp: Ccmp) {
        self.wipe_keys();
        self.bssid = bssid;
        self.ccmp = ccmp;
        self.pn = 0;
        self.seq = 0;
        self.associated = true;
        self.chip_keyed = false;
        self.replay = Replay::new();
        self.group_keys = [const { None }; 4];
        self.supplicant = None;
        self.pmf = PmfState::OFF;
        self.watch = Watch::new();
    }

    /// Drop the association; the station stops framing until it associates
    /// again, and forgets the keys and the supplicant.
    pub fn deassociate(&mut self) {
        self.associated = false;
        self.wipe_keys();
        self.pmf = PmfState::OFF;
    }

    /// The BSSID of the current association.
    pub fn bssid(&self) -> MacAddr {
        self.bssid
    }

    /// Whether the station is associated and carrying data.
    pub fn is_associated(&self) -> bool {
        self.associated
    }

    /// An exchange the link must see through on its channel: an SA Query
    /// awaiting the AP's answer, or a pairwise rekey between message 1 and
    /// the new keys going in.
    pub fn busy(&self) -> bool {
        let rekeying = self.supplicant.as_ref().is_some_and(|s| s.state() == State::PtkDerived);
        self.pmf.query.is_some() || rekeying
    }

    /// The station MAC address.
    pub fn mac(&self) -> MacAddr {
        self.our_mac
    }
}
