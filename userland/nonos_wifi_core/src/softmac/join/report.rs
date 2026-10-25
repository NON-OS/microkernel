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

//! How a join ended and how far it got. The counters turn a bare timeout
//! into a diagnosis the panel can show: one frame sent and none received
//! points at transmit, data frames with no EAPOL among them at framing, a
//! deauthentication after the first EAPOL at a handshake frame lost here.

use super::counters::Counters;
use crate::mlme::MlmeFailure;
use crate::pmksa::Pmksa;
use crate::rsn::{Selection, TemporalKey};
use crate::wpa::akm::Akm;
use crate::wpa::supplicant::Supplicant;

// Built once per join and moved straight into the session, so the size gap
// between Joined and the bare endings costs nothing worth a box.
#[allow(clippy::large_enum_variant)]
pub enum Outcome {
    /// Associated: the AP, its channel, the pairwise key and the supplicant
    /// that keeps answering the AP's group key handshakes.
    Joined { bssid: [u8; 6], channel: u8, tk: TemporalKey, akm: Akm, supplicant: Supplicant },
    /// Refused, or the handshake broke; `Report::failure` says where.
    Refused,
    /// The AP stopped answering before the join finished.
    TimedOut,
}

pub struct Report {
    pub outcome: Outcome,
    pub counters: Counters,
    /// The state reached: 0 scanning to 4 connected, 5 failed.
    pub state: u8,
    pub failure: Option<MlmeFailure>,
    pub akm: Option<Akm>,
    /// The join ran on a cached PMKSA instead of SAE.
    pub used_pmksa: bool,
    /// The PMKSA a connected SAE join holds, to cache.
    pub pmksa: Option<Pmksa>,
    /// The security the join selected from the beacon.
    pub selection: Option<Selection>,
    /// The AP accepted the (re)association, whatever followed.
    pub accepted: bool,
    /// The group management cipher suite the AP's beacon named.
    pub group_mgmt: Option<u32>,
    /// The Transition Disable bits message 3 carried, for the caller to
    /// keep with the saved network.
    pub transition_disable: Option<u8>,
}
