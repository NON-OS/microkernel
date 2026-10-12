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
//! One NDP32 of a received NTB32 (ntb32_in.rs): its header checked inside
//! the transfer, its datagrams queued up to the first null or bad entry,
//! as many as `room` allows.

use alloc::collections::VecDeque;

use nonos_usbnet::nic::ETH_HEADER;

use super::verify::{le16, le32};

const NDP32_NOCRC_SIGN: u32 = 0x306D_636E;
pub const NDP32_HEADER: usize = 16;
const ENTRY: usize = 8;
/// An NDP32 with one entry and its terminator.
const NDP32_ONE: usize = NDP32_HEADER + 2 * ENTRY;

/// The next NDP's index (0 at the end), or `None` for a malformed NDP.
pub fn ndp32(
    raw: &[u8],
    at: usize,
    rx_max: usize,
    room: usize,
    out: &mut VecDeque<(usize, usize)>,
) -> Option<usize> {
    if at.checked_add(NDP32_HEADER)? > raw.len() {
        return None;
    }
    let len = le16(raw, at + 4);
    if len < NDP32_ONE {
        return None;
    }
    let n = (len - NDP32_HEADER) / ENTRY - 1;
    if at + NDP32_HEADER + n * ENTRY > raw.len() {
        return None;
    }
    // Only "ncm0": CRC mode is off, as for the NTB16's "NCM0".
    if le32(raw, at) == NDP32_NOCRC_SIGN {
        for x in 0..n {
            let e = at + NDP32_HEADER + ENTRY * x;
            let (index, dlen) = (le32(raw, e) as usize, le32(raw, e + 4) as usize);
            let inside = index <= raw.len() && dlen <= raw.len() - index;
            if out.len() >= room || index == 0 || dlen < ETH_HEADER || dlen > rx_max || !inside {
                break;
            }
            out.push_back((index, dlen));
        }
    }
    Some(le32(raw, at + 8) as usize)
}
