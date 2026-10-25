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

//! The shell's brightness keys, carried to the display driver that serves
//! F 12's bytes (the kernel lets only the compositor reach a driver): which
//! drivers serve them, the panels a BRIGHTNESS answer lists, the one a key
//! moves (the focused output's, else the first listed) and the level it
//! moves to, the next one listed either way. Pure, so the host proofs pin
//! it (compositor_proofs brightness_tests).

use super::layout_rule::Plan;

/// The drivers that answer F 12's bytes on their port.
pub const SERVES: &[&str] = &["driver.intel_display0"];
pub const OP_BRIGHTNESS: u8 = 3;
pub const OP_SET_BRIGHTNESS: u8 = 4;
pub const OP_SET_VCP: u8 = 24;

pub fn serves(name: &str) -> bool {
    SERVES.contains(&name)
}

/// A panel as BRIGHTNESS lists it: its identity, its level now in percent
/// (0xFF unknown), and the levels it takes, ascending.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Panel<'a> {
    pub identity: u32,
    pub now: u8,
    pub levels: &'a [u8],
}

/// The driver's name for output `output`'s panel.
pub fn identity(output: u16) -> u32 {
    0x8000_0000 | u32::from(output)
}

/// The output the canvas point `x` shows on: the slice holding it under
/// `plan`, else output 0; `outputs` names each slice's output.
pub fn output_at(plan: Option<&Plan>, outputs: &[u16], x: u32) -> u16 {
    let Some(p) = plan else { return 0 };
    let i = p.slices.iter().take(p.n).position(|s| x >= s.x && x < s.x + s.w);
    i.and_then(|i| outputs.get(i).copied()).unwrap_or(0)
}

/// The panel a key moves: `output`'s, else the first listed.
pub fn target<'a>(panels: &[Panel<'a>], output: u16) -> Option<Panel<'a>> {
    panels.iter().find(|p| p.identity == identity(output)).or(panels.first()).copied()
}

/// The level a key moves `p` to: the next listed above its level now, or
/// below; None at either end, or when the level now is unknown.
pub fn next(p: &Panel, up: bool) -> Option<u8> {
    if p.now > 100 {
        return None;
    }
    match up {
        true => p.levels.iter().copied().find(|l| *l > p.now),
        false => p.levels.iter().copied().rev().find(|l| *l < p.now),
    }
}
