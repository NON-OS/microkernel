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

//! An associated session: the network, the station engine keyed for it, the
//! frames waiting for the stack, and when the AP was last heard.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use crate::regdom::Band;
use crate::station::LinkStation;
use crate::wpa::supplicant::Supplicant;

/// Frames held for net_core between polls; past this the oldest go.
pub const RX_QUEUE: usize = 64;
/// No frame from the AP for this long (a hundred beacon intervals) means the
/// AP is gone: the session ends and the scan resumes.
pub const LOSS_MS: u64 = 10_000;

pub struct Session {
    pub bssid: [u8; 6],
    pub ssid: [u8; 32],
    pub ssid_len: u8,
    /// The AKM suite type that ran: 2 WPA2-PSK, 6 PSK-SHA256, 8 SAE.
    pub akm: u8,
    pub band: Band,
    pub channel: u8,
    /// Management frame protection was negotiated.
    pub pmf: bool,
    pub(super) station: LinkStation,
    pub(super) queue: VecDeque<Vec<u8>>,
    pub(super) heard_ms: u64,
}

/// What a session starts from.
pub struct Keys {
    pub tk: [u8; 16],
    pub supplicant: Supplicant,
}

impl Session {
    pub fn ssid(&self) -> &[u8] {
        &self.ssid[..self.ssid_len as usize]
    }

    pub fn is_up(&self) -> bool {
        self.station.is_associated()
    }
}
