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

//! What the firmware contexts need to know of the network, read from its
//! beacon (or probe response): the BSSID, the channel (from the DS parameter
//! element, else the channel the frame was heard on, since a 5 GHz beacon
//! may carry none), the beacon interval and capability from the fixed
//! fields, the DTIM period from the TIM element, and the rates. Every field
//! is read through the shared parser's bounds-checked walk.

use nonos_wifi_core::dot11::ies::{ap_rates, beacon_tags, find_element};
use nonos_wifi_core::dot11::parse::parse_beacon;

use super::super::station::rates::{bss_rates, BssRates};

/// `WLAN_EID_TIM`.
const EID_TIM: u8 = 5;
/// The beacon interval's offset: the MAC header, then the 8-byte timestamp.
const BEACON_INT_AT: usize = 24 + 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Target {
    pub bssid: [u8; 6],
    pub channel: u8,
    pub beacon_int: u16,
    pub capability: u16,
    pub dtim_period: u8,
    pub rates: BssRates,
}

/// The network a beacon heard on `heard_on` describes, or `None` for a frame
/// that is not a beacon or probe response, or names no usable channel.
pub fn target(beacon: &[u8], heard_on: u8) -> Option<Target> {
    let info = parse_beacon(beacon)?;
    let tags = beacon_tags(beacon)?;
    let channel = if info.channel != 0 { info.channel } else { heard_on };
    if channel == 0 {
        return None;
    }
    let beacon_int = u16::from_le_bytes([*beacon.get(BEACON_INT_AT)?, *beacon.get(BEACON_INT_AT + 1)?]);
    let dtim_period = find_element(tags, EID_TIM).and_then(|e| e.get(3).copied()).unwrap_or(0);
    let r = ap_rates(tags);
    let mut octets = [0u8; 16];
    let n = r.supported().len() + r.extended().len();
    octets[..r.supported().len()].copy_from_slice(r.supported());
    octets[r.supported().len()..n].copy_from_slice(r.extended());
    Some(Target {
        bssid: info.bssid,
        channel,
        beacon_int,
        capability: info.capability,
        dtim_period,
        rates: bss_rates(&octets[..n], channel),
    })
}
