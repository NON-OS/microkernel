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

//! Each display's brightness, shown on Appearance and never set from it:
//! the compositor's BRIGHTNESS_GET through the shell, a count u32 then per
//! panel its identity u32 (the output, with the top bit set) and its level
//! in percent u32. A level past 100, or an output past four, is not shown.

pub const ANSWER_LEN: usize = 28;

/// The levels by output, from an answer.
pub fn levels(answer: &[u8; ANSWER_LEN]) -> [Option<u8>; 4] {
    let mut out = [None; 4];
    let word =
        |i: usize| u32::from_le_bytes([answer[i], answer[i + 1], answer[i + 2], answer[i + 3]]);
    for p in 0..(word(0) as usize).min(3) {
        let (identity, level) = (word(4 + p * 8), word(8 + p * 8));
        let output = (identity & 0x7FFF_FFFF) as usize;
        if let (Some(slot), Ok(l @ 0..=100)) = (out.get_mut(output), u8::try_from(level)) {
            *slot = Some(l);
        }
    }
    out
}
