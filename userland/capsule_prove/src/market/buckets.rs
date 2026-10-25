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

//! The owner's measuring line (plan/14, plan/32): `[PROVE] syscall buckets
//! ok=` and 130 hex digits, bit b set when H[b][0] > 0, byte b/8 bit b%8,
//! byte 0 first. Said through MkDebug, which only a capsule granted Debug (a
//! capsule-serial-debug kernel) is heard on; a release build never says it.

use nonos_proof_abi::{BUCKETS, HIST_WORDS};

pub const PREFIX: &[u8] = b"[PROVE] syscall buckets ok=";
pub const LINE_LEN: usize = 27 + 130;

pub fn line(hist: &[u32; HIST_WORDS]) -> [u8; LINE_LEN] {
    let mut set = [0u8; 65];
    (0..BUCKETS).filter(|&b| hist[2 * b] > 0).for_each(|b| set[b / 8] |= 1 << (b % 8));
    let mut out = [0u8; LINE_LEN];
    out[..PREFIX.len()].copy_from_slice(PREFIX);
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for (i, byte) in set.iter().enumerate() {
        out[PREFIX.len() + 2 * i] = HEX[usize::from(byte >> 4)];
        out[PREFIX.len() + 2 * i + 1] = HEX[usize::from(byte & 15)];
    }
    out
}
