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

//! The displays as Appearance shows them: how many, mirrored or extended
//! (chosen, and shown now), output 0's scale now and each display's chosen
//! scale; and how a key moves the rows. Pure, so the proofs drive it.

use crate::settings::schema::access_knob::Knob;

/// The scales the compositor takes, in quarters, 0 its panel's own rule:
/// automatic, then 1, 1.25, 1.5, 2, 3 and 4 times.
pub const SCALES: [u32; 7] = [0, 4, 5, 6, 8, 12, 16];
pub const OUTPUTS: u32 = 4;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Shown {
    pub outputs: u32,
    pub extend: bool,
    pub extended: bool,
    pub scale_q: u32,
    pub chosen: [u32; 4],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScreenPage {
    pub shown: Shown,
    /// The display the scale row is for.
    pub output: u32,
    /// Whether `shown` came from the compositor.
    pub read: bool,
    /// Each display's brightness now in percent (`levels.rs`), read only.
    pub levels: [Option<u8>; 4],
}

fn step_in(v: u32, n: u32, delta: i32) -> u32 {
    (v as i64 + delta as i64).clamp(0, n.max(1) as i64 - 1) as u32
}

/// Move `knob` by `delta` steps; whether anything changed.
pub fn step(page: &mut ScreenPage, knob: Knob, delta: i32) -> bool {
    let before = *page;
    let s = &mut page.shown;
    match knob {
        Knob::Arrange => s.extend = step_in(s.extend as u32, 2, delta) == 1,
        Knob::Screen => page.output = step_in(page.output, s.outputs.min(OUTPUTS), delta),
        Knob::Scale => {
            let slot = &mut s.chosen[page.output.min(OUTPUTS - 1) as usize];
            let i = SCALES.iter().position(|q| q == slot).unwrap_or(0) as u32;
            *slot = SCALES[step_in(i, SCALES.len() as u32, delta) as usize];
        }
        _ => {}
    }
    *page != before
}
