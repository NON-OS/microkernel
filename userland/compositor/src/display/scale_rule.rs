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

//! An output's scale, in quarters of a screen pixel per canvas pixel: the
//! shell's choice among the steps a desktop is laid out for (1, 1.25, 1.5,
//! 2, 3 and 4 times), else the panel rule the kernel's boot console keeps
//! (`canvas_scale::scale_for_panel`, whole steps only); and the canvas an
//! output of a given size gets at it. Pure, so the host proofs pin it.

/// The steps the shell may choose, in quarters.
pub const STEPS: [u32; 6] = [4, 5, 6, 8, 12, 16];

/// The scale to use: the shell's step when it chose one, else the panel's.
pub fn chosen(shell: Option<u32>, panel_whole: u32) -> u32 {
    shell.filter(|q| STEPS.contains(q)).unwrap_or(panel_whole.clamp(1, 4) * 4)
}

/// The canvas a `w` x `h` screen gets at `q` quarters.
pub fn canvas_for(w: u32, h: u32, q: u32) -> (u32, u32) {
    let q = q.max(4) as u64;
    ((w as u64 * 4 / q) as u32, (h as u64 * 4 / q) as u32)
}

/// The shape's whole step for the device's cursor plane at `q`.
pub fn whole(q: u32) -> u32 {
    (q.max(4) + 2) / 4
}
