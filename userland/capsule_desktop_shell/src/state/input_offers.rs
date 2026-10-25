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

//! What the displays offer the input menu, as the compositor's
//! INPUT_SOURCES answers: a count u32, then per display its identity u32,
//! the input now u16, n u8, a zero byte and eight values. A count or an n
//! past what the answer can hold is cut to it.

pub const MAX_DISPLAYS: usize = 3;
pub const MAX_INPUTS: usize = 8;
pub const ANSWER_LEN: usize = 4 + MAX_DISPLAYS * 16;

/// What one display offers: its panel, the input it shows now (an MCCS
/// value) and the inputs it lists.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Offer {
    pub identity: u32,
    pub now: u16,
    pub inputs: [u8; MAX_INPUTS],
    pub n: u8,
}

/// One row: a display (its place in the menu) and one of its inputs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row {
    pub display: usize,
    pub input: u8,
    pub showing: bool,
}

/// The displays the answer names, in order.
pub fn read(answer: &[u8; ANSWER_LEN]) -> ([Offer; MAX_DISPLAYS], usize) {
    let mut out = [Offer::default(); MAX_DISPLAYS];
    let count = u32::from_le_bytes([answer[0], answer[1], answer[2], answer[3]]) as usize;
    let count = count.min(MAX_DISPLAYS);
    for (o, e) in out.iter_mut().zip(answer[4..].chunks_exact(16)).take(count) {
        o.identity = u32::from_le_bytes([e[0], e[1], e[2], e[3]]);
        o.now = u16::from_le_bytes([e[4], e[5]]);
        o.n = e[6].min(MAX_INPUTS as u8);
        o.inputs.copy_from_slice(&e[8..16]);
    }
    (out, count)
}
