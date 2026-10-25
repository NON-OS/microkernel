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

//! The station itself: its association facts and its counters.

use super::pmf_state::PmfState;
use super::replay::Replay;
use super::watch::Watch;
use super::Ccmp;
use crate::dot11::header::MacAddr;
use crate::rsn::TemporalKey;
use crate::wpa::supplicant::Supplicant;

/// The per-station data path: association facts plus the transmit counters.
pub struct LinkStation {
    pub(super) our_mac: MacAddr,
    pub(super) bssid: MacAddr,
    pub(super) ccmp: Ccmp,
    pub(super) pn: u64,
    pub(super) seq: u16,
    pub(super) associated: bool,
    pub(super) replay: Replay,
    /// Group keys by index, for frames the chip did not decrypt.
    pub(super) group_keys: [Option<TemporalKey>; 4],
    /// The join's supplicant, answering the AP's later key handshakes.
    pub(super) supplicant: Option<Supplicant>,
    pub(super) pmf: PmfState,
    /// Under `Ccmp::Hardware`, whether the driver says the chip holds the keys.
    pub(super) chip_keyed: bool,
    /// The clock, the access point's liveness and the station's own query.
    pub(super) watch: Watch,
}

impl LinkStation {
    /// A fresh station that refuses to transmit until `associate`.
    pub fn new(our_mac: MacAddr) -> Self {
        Self {
            our_mac,
            bssid: [0u8; 6],
            ccmp: Ccmp::Hardware,
            pn: 0,
            seq: 0,
            associated: false,
            replay: Replay::new(),
            group_keys: [const { None }; 4],
            supplicant: None,
            pmf: PmfState::OFF,
            chip_keyed: false,
            watch: Watch::new(),
        }
    }
}
