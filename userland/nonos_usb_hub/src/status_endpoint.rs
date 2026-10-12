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

//! The hub's status change endpoint: the interrupt IN endpoint of its hub
//! class interface (USB 2.0 section 11.12.1). Configuring it claims the
//! root port for this driver, so the storage driver does not take the hub.
//! Pure, so the walk is proven on the host.

const DT_INTERFACE: u8 = 0x04;
const DT_ENDPOINT: u8 = 0x05;
const CLASS_HUB: u8 = 0x09;
const INTERRUPT: u8 = 0x03;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusEndpoint {
    pub address: u8,
    pub max_packet: u16,
    pub interval: u8,
}

/// The first interrupt IN endpoint of a hub interface in configuration
/// `raw`, whose wTotalLength the caller has checked.
pub fn status_endpoint(raw: &[u8]) -> Option<StatusEndpoint> {
    let mut in_hub = false;
    let mut i = raw.first().map_or(0, |&l| l as usize).max(2);
    while i + 2 <= raw.len() {
        let len = raw[i] as usize;
        if len < 2 || i + len > raw.len() {
            return None;
        }
        let d = &raw[i..i + len];
        match d[1] {
            DT_INTERFACE => in_hub = len >= 9 && d[5] == CLASS_HUB,
            DT_ENDPOINT if in_hub && len >= 7 && d[2] & 0x80 != 0 && d[3] & 0x3 == INTERRUPT => {
                let max_packet = u16::from_le_bytes([d[4], d[5]]) & 0x07ff;
                return Some(StatusEndpoint { address: d[2], max_packet, interval: d[6] });
            }
            _ => {}
        }
        i += len;
    }
    None
}
