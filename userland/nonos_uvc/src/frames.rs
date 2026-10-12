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
//! What the hub class needs of a USB host controller driver and of the
//! The streaming interface's alternate settings: each one's number and
//! its isochronous IN endpoint, with what one service interval carries
//! (wMaxPacketSize bits 10:0, times one more than bits 12:11 at high
//! speed).

use alloc::vec::Vec;

use crate::alt::Alt;

const DT_INTERFACE: u8 = 0x04;
const DT_ENDPOINT: u8 = 0x05;
const ISOCHRONOUS: u8 = 0x01;

pub(crate) fn alternate(raw: &[u8], streaming: u8) -> Vec<Alt> {
    let mut out = Vec::new();
    let mut alt: Option<u8> = None;
    let mut i = raw.first().map_or(0, |&l| l as usize).max(2);
    while i + 2 <= raw.len() {
        let len = raw[i] as usize;
        let Some(d) = raw.get(i..i + len).filter(|_| len >= 2) else { break };
        match d[1] {
            DT_INTERFACE if len >= 9 => alt = (d[2] == streaming).then_some(d[3]),
            DT_ENDPOINT if len >= 7 && d[2] & 0x80 != 0 && d[3] & 0x3 == ISOCHRONOUS => {
                if let Some(setting) = alt.filter(|&a| a != 0) {
                    let mps = u16::from_le_bytes([d[4], d[5]]);
                    let per = (mps & 0x7FF) as u32 * (((mps >> 11) & 0x3) as u32 + 1);
                    out.push(Alt { setting, endpoint: d[2], max_packet: mps, per_interval: per });
                }
            }
            _ => {}
        }
        i += len;
    }
    out
}
