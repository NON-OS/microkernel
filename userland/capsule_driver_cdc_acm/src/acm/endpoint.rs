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
//! The endpoint an ACM function's notifications come on: the first
//! interrupt IN endpoint of its communications interface, alternate 0.

use nonos_usbnet::desc::{walk, ENDPOINT, INTERFACE};

const INTERRUPT: u8 = 0x03;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Notify {
    pub address: u8,
    pub max_packet: u16,
    pub interval: u8,
}

/// The first interrupt IN endpoint of interface `number`, alternate 0.
pub fn notify(raw: &[u8], number: u8) -> Option<Notify> {
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
