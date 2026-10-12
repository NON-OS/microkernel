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
//! A communications interface's notification endpoint: its first
//! interrupt IN endpoint, alternate 0 (CDC 1.2, 3.3.1, as recalled,
//! unverified). NCM's link and RNDIS's responses come on it.

use super::kinds::{ENDPOINT, INTERFACE};
use super::walk::walk;
use crate::bus::Notify;

const INTERRUPT: u8 = 0x03;

pub fn notify_ep(raw: &[u8], number: u8) -> Option<Notify> {
    let mut inside = false;
    walk(raw).find_map(|d| {
        if d[1] == INTERFACE && d.len() >= 4 {
            inside = d[2] == number && d[3] == 0;
        }
        let ours = inside && d[1] == ENDPOINT && d.len() >= 7;
        (ours && d[2] & 0x80 != 0 && d[3] & 0x3 == INTERRUPT).then(|| Notify {
            address: d[2],
            max_packet: u16::from_le_bytes([d[4], d[5]]),
            interval: d[6],
        })
    })
}
