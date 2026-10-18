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

//! OUTPUT_ARRANGEMENT's answer (NCMP 0x0011), what Settings shows of the
//! displays: how many are shown, the shell's choice (0 mirror, 1 extend),
//! whether the canvas is extended now (a choice to extend waits for a
//! second display), output 0's scale now, and each output's chosen scale
//! (0 the panel's own rule), in quarters: eight u32s.

pub const ARRANGEMENT_LEN: usize = 32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Arrangement {
    pub outputs: u32,
    pub extend: bool,
    pub extended: bool,
    pub scale_q: u32,
    pub chosen: [Option<u32>; 4],
}

pub fn encode(a: &Arrangement) -> [u8; ARRANGEMENT_LEN] {
    let c = a.chosen.map(|q| q.unwrap_or(0));
    let words = [a.outputs, a.extend as u32, a.extended as u32, a.scale_q, c[0], c[1], c[2], c[3]];
    let mut out = [0u8; ARRANGEMENT_LEN];
    for (i, w) in words.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
    }
    out
}
