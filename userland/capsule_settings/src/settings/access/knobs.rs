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

//! The page's values and how a key moves them: Left and Right a step, held
//! inside each knob's range; the display among the four the compositor
//! serves. Pure, so the proofs drive it.

use crate::settings::schema::access_knob::Knob;

pub const OUTPUTS: u32 = 4;
pub const FILTERS: u32 = 5;
/// 1x (off), 1.5x, 2x, 3x, 4x, 6x and 8x, in quarters as the compositor
/// takes them.
pub const MAG_STEPS: [u32; 7] = [4, 6, 8, 12, 16, 24, 32];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Knobs {
    pub filter: u32,
    pub contrast: u32,
    pub magnify: u32,
    pub follow: u32,
}

impl Default for Knobs {
    fn default() -> Self {
        Knobs { filter: 0, contrast: 0, magnify: 4, follow: 0 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AccessPage {
    pub output: u32,
    pub knobs: Knobs,
    /// Whether `knobs` came from the compositor for `output`.
    pub read: bool,
}

fn step_in(v: u32, n: u32, delta: i32) -> u32 {
    (v as i64 + delta as i64).clamp(0, n as i64 - 1) as u32
}

/// Move `knob` by `delta` steps; whether anything changed.
pub fn step(page: &mut AccessPage, knob: Knob, delta: i32) -> bool {
    let before = *page;
    let k = &mut page.knobs;
    match knob {
        Knob::Output => page.output = step_in(page.output, OUTPUTS, delta),
        Knob::Filter => k.filter = step_in(k.filter, FILTERS, delta),
        Knob::Contrast => k.contrast = step_in(k.contrast, 2, delta),
        Knob::Follow => k.follow = step_in(k.follow, 2, delta),
        Knob::Magnify => {
            let i = MAG_STEPS.iter().position(|&m| m == k.magnify).unwrap_or(0);
            k.magnify = MAG_STEPS[step_in(i as u32, MAG_STEPS.len() as u32, delta) as usize];
        }
        // The displays' rows are `screens`'.
        Knob::Arrange | Knob::Screen | Knob::Scale => {}
    }
    *page != before
}
