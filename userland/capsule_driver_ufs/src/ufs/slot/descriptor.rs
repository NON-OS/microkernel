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

//! The transfer request descriptor for one request (layout in
//! facts::utp). No completion interrupt is asked for: the engine polls the
//! doorbell.

use super::layout::{PRDT_AT, RSP_AT};
use crate::ufs::facts::utp::*;

/// The data direction, as DW0 encodes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    None = 0,
    ToDevice = 1,
    FromDevice = 2,
}

/// The eight dwords for a request whose command descriptor is at `ucd_bus`
/// with `prdt` entries: type and direction, the OCS placeholder, the
/// descriptor's address, then the response and PRDT offsets and lengths,
/// all in dwords.
pub fn descriptor(cmd_type: u32, dir: Dir, ucd_bus: u64, prdt: u16) -> [u8; UTRD_LEN] {
    let dwords = |bytes: usize| (bytes / 4) as u32;
    let words: [u32; 8] = [
        cmd_type << TYPE_SHIFT | (dir as u32) << DIR_SHIFT,
        0,
        OCS_INVALID,
        0,
        ucd_bus as u32,
        (ucd_bus >> 32) as u32,
        dwords(RSP_AT) << 16 | dwords(UPIU_AREA),
        dwords(PRDT_AT) << 16 | prdt as u32,
    ];
    let mut out = [0u8; UTRD_LEN];
    for (chunk, w) in out.chunks_exact_mut(4).zip(words) {
        chunk.copy_from_slice(&w.to_le_bytes());
    }
    out
}

/// The overall command status the host left; 0 is success.
pub fn ocs(desc: &[u8; UTRD_LEN]) -> u8 {
    desc[OCS_AT]
}
