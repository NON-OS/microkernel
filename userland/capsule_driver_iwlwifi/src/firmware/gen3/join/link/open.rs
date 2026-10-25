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
//! A link made from a finished join, and the deauthentication it leaves with.

use super::state::{Link, LinkStats, REASON_LEAVING};
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use nonos_wifi_core::dot11::auth::deauth_frame;
use nonos_wifi_core::station::{Ccmp, LinkStation};
use nonos_wifi_core::wpa::supplicant::Supplicant;

impl Link {
    /// The link to `bssid` keyed with `tk`, taking over the supplicant the
    /// join finished with.
    pub fn new(us: [u8; 6], bssid: [u8; 6], tk: [u8; 16], sup: Supplicant, data_rate: u32, mgmt_rate: u32) -> Self {
        let mut station = LinkStation::new(us);
        station.associate(bssid, Ccmp::SoftwareTxHwRx { tk });
        let pmf = sup.pmf();
        station.set_supplicant(sup);
        Self {
            station,
            ready: VecDeque::new(),
            pmf,
            left: None,
            silent: false,
            new_group_key: None,
            new_pairwise_key: None,
            stats: LinkStats::default(),
            data_rate,
            mgmt_rate,
        }
    }
    /// The deauthentication to send when leaving, protected under the
    /// pairwise key when management frame protection is on.
    pub fn deauth(&mut self) -> Option<Vec<u8>> {
        if !self.station.is_associated() {
            return None;
        }
        let frame = deauth_frame(self.station.mac(), self.station.bssid(), 0, REASON_LEAVING);
        if self.pmf {
            self.station.protect_mgmt(&frame)
        } else {
            Some(frame)
        }
    }
}
