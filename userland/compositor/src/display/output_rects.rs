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

//! Where each output sits on the canvas, for the shell's lock screen, which
//! draws its clock and field on every one. Mirrored, or one output: the
//! whole canvas. Extended: each output's slice, left to right.

use super::layout_rule::{Plan, MAX_SLICES};

pub type Rect = (u32, u32, u32, u32);

pub fn rects(plan: Option<&Plan>, canvas: (u32, u32)) -> ([Rect; MAX_SLICES], usize) {
    let mut out = [(0, 0, 0, 0); MAX_SLICES];
    match plan {
        Some(p) if p.n > 0 => {
            for (slot, s) in out.iter_mut().zip(p.slices.iter().take(p.n)) {
                *slot = (s.x, 0, s.w, s.h);
            }
            (out, p.n.min(MAX_SLICES))
        }
        _ => {
            out[0] = (0, 0, canvas.0, canvas.1);
            (out, 1)
        }
    }
}

/// The answer's length: always four slots, so it has one length.
pub const REPLY_LEN: usize = 4 + 16 * MAX_SLICES;

/// `n u32`, then x, y, width, height (u32 each) for each output; the slots
/// past `n` are zero.
pub fn encode(rects: &[Rect]) -> [u8; REPLY_LEN] {
    let mut out = [0u8; REPLY_LEN];
    let n = rects.len().min(MAX_SLICES);
    out[0..4].copy_from_slice(&(n as u32).to_le_bytes());
    for (i, r) in rects.iter().take(n).enumerate() {
        let at = 4 + i * 16;
        for (j, v) in [r.0, r.1, r.2, r.3].into_iter().enumerate() {
            out[at + j * 4..at + j * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
    }
    out
}
