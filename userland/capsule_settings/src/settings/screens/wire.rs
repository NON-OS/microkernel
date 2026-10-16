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

//! The bodies the shell passes to the compositor unchanged: OUTPUT_LAYOUT
//! takes the mode (0 mirror, 1 extend), OUTPUT_SCALE the display and its
//! quarters, and OUTPUT_ARRANGEMENT answers eight u32s (`arrange::Shown`).

use super::arrange::{Shown, OUTPUTS, SCALES};

pub fn layout(extend: bool) -> [u8; 4] {
    (extend as u32).to_le_bytes()
}

pub fn scale(output: u32, q: u32) -> Option<[u8; 8]> {
    let mut b = [0u8; 8];
    b[..4].copy_from_slice(&output.to_le_bytes());
    b[4..].copy_from_slice(&q.to_le_bytes());
    (output < OUTPUTS && SCALES.contains(&q)).then_some(b)
}

/// The arrangement as answered, or None for anything out of its range.
pub fn shown(b: &[u8]) -> Option<Shown> {
    let u = |i: usize| Some(u32::from_le_bytes(b.get(i * 4..i * 4 + 4)?.try_into().ok()?));
    let chosen = [u(4)?, u(5)?, u(6)?, u(7)?];
    let s =
        Shown { outputs: u(0)?, extend: u(1)? == 1, extended: u(2)? == 1, scale_q: u(3)?, chosen };
    let flags = u(1)? <= 1 && u(2)? <= 1;
    let scales = SCALES[1..].contains(&s.scale_q) && chosen.iter().all(|q| SCALES.contains(q));
    (b.len() == 32 && (1..=OUTPUTS).contains(&s.outputs) && flags && scales).then_some(s)
}
