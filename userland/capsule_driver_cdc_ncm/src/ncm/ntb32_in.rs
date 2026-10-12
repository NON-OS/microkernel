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
//! The datagrams of one received NTB32 (NCM 1.0, 3.2.2 and 3.3.2, as
//! recalled, unverified): an NTH32 ("ncmh", a 16-byte header with 32-bit
//! block length and first NDP index), then NDP32s ("ncm0", ndp32.rs),
//! checked as the NTB16's are: every read inside the transfer, a chain that
//! loops ends, what is malformed ends what is taken from it.

use alloc::collections::VecDeque;

use super::ndp32::{ndp32, NDP32_HEADER};
use super::verify::le32;

pub const NTH32_SIGN: u32 = 0x686D_636E;
const NTH32_LEN: usize = 16;
const MAX_NDPS: usize = 50;

/// Queue each datagram of the NTB32 in `raw`, at most `room` of them.
pub fn datagrams32(raw: &[u8], rx_max: usize, room: usize, out: &mut VecDeque<(usize, usize)>) {
    if raw.len() < NTH32_LEN + NDP32_HEADER || le32(raw, 0) != NTH32_SIGN {
        return;
    }
    if le32(raw, 8) as usize > rx_max {
        return;
    }
    let (mut at, mut seen) = (le32(raw, 12) as usize, [0usize; MAX_NDPS]);
    for i in 0..MAX_NDPS {
        if seen[..i].contains(&at) || at < NTH32_LEN {
            return;
        }
        seen[i] = at;
        match ndp32(raw, at, rx_max, room, out) {
            Some(0) | None => return,
            Some(next) => at = next,
        }
    }
}
