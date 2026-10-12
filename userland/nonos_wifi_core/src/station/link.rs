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

//! The station itself, and how an association starts.

use super::replay::Replay;
use super::watch::Watch;
use super::Ccmp;
use crate::dot11::header::MacAddr;
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
    pub(super) group_keys: [Option<[u8; 16]>; 4],
    /// The supplicant the join finished with, which answers the AP's group
    /// key handshakes and repeated message 3s.
    pub(super) supplicant: Option<Supplicant>,
    pub(super) watch: Watch,
}

impl LinkStation {
    /// A fresh, unassociated station with the given MAC. It refuses to transmit
    /// until `associate` records a BSSID and CCMP mode.
    pub fn new(our_mac: MacAddr) -> Self {
        Self {
            our_mac,
            bssid: [0u8; 6],
            ccmp: Ccmp::Hardware,
            pn: 0,
            seq: 0,
            associated: false,
            replay: Replay::new(),
            group_keys: [None; 4],
            supplicant: None,
            watch: Watch::new(),
        }
    }

    /// Record association to `bssid` with the given CCMP mode, resetting the
    /// sequence and packet-number counters for the new session.
    pub fn associate(&mut self, bssid: MacAddr, ccmp: Ccmp) {
        self.bssid = bssid;
        self.ccmp = ccmp;
        self.pn = 0;
        self.seq = 0;
        self.associated = true;
        self.replay = Replay::new();
        self.group_keys = [None; 4];
        self.supplicant = None;
        self.watch = Watch::new();
    }
}
