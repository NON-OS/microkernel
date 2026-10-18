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

//! Requests that carry one small setting: a yes or no, a temperature, a
//! scale step, a privacy indicator.

use super::read::words;
use crate::display::night_rule::WARMEST_K;
use crate::display::scale_rule::STEPS;
use crate::protocol::{
    BRIGHTNESS_STEP_REQ_LEN, INDICATOR_REQ_LEN, LID_REQ_LEN, NIGHT_LIGHT_REQ_LEN,
    OUTPUT_LAYOUT_REQ_LEN, OUTPUT_SCALE_REQ_LEN,
};

fn flag<const LEN: usize>(body: &[u8]) -> Option<bool> {
    let [v] = words::<1, LEN>(body)?;
    (v <= 1).then_some(v == 1)
}

/// LID: true when the lid closed.
pub fn lid(body: &[u8]) -> Option<bool> {
    flag::<LID_REQ_LEN>(body)
}

/// BRIGHTNESS_STEP: true for a step up.
pub fn brightness_step(body: &[u8]) -> Option<bool> {
    flag::<BRIGHTNESS_STEP_REQ_LEN>(body)
}

/// OUTPUT_LAYOUT: true to extend across the outputs, false to mirror.
pub fn output_layout(body: &[u8]) -> Option<bool> {
    flag::<OUTPUT_LAYOUT_REQ_LEN>(body)
}

/// INDICATOR: the kind, which the indicator table judges, and on or off.
pub fn indicator(body: &[u8]) -> Option<(u32, bool)> {
    let [kind, on] = words::<2, INDICATOR_REQ_LEN>(body)?;
    (on <= 1).then_some((kind, on == 1))
}

/// NIGHT_LIGHT: 0 for off, else a temperature no warmer than the floor.
pub fn night_light(body: &[u8]) -> Option<u32> {
    let [k] = words::<1, NIGHT_LIGHT_REQ_LEN>(body)?;
    (k == 0 || k >= WARMEST_K).then_some(k)
}

/// OUTPUT_SCALE: the output, and its scale in quarters or None to follow
/// the panel. Whether the output exists is the caller's to judge.
pub fn output_scale(body: &[u8]) -> Option<(usize, Option<u32>)> {
    let [out, q] = words::<2, OUTPUT_SCALE_REQ_LEN>(body)?;
    (q == 0 || STEPS.contains(&q)).then_some((out as usize, (q != 0).then_some(q)))
}
