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

//! Terminal text metrics for crisp TrueType rendering. The body is monospace,
//! so a single measured cell advance lays out the whole grid, and the zoom
//! level picks the point size instead of an integer bitmap multiple.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;

/// Logical point size at the smallest zoom level less one step; each zoom
/// level from 1 to 6 adds `ZOOM_STEP`, and level 2 is the readable default.
const BASE_PX: u32 = 11;
/// Logical points one zoom level adds.
const ZOOM_STEP: u32 = 2;
/// Logical leading under the point size, which makes the row height.
const LEADING: u32 = 5;

// The monospace body metrics for one frame: point size, measured cell advance,
// and row height. Bundled so the draw functions take one value instead of three.
#[derive(Clone, Copy)]
pub struct Metrics {
    pub px: f32,
    pub adv: u32,
    pub lh: u32,
}

impl Metrics {
    pub fn new(fb: &PaintBuffer, font_scale: u32) -> Self {
        let logical = BASE_PX + font_scale.clamp(1, 6) * ZOOM_STEP;
        let px = scale().font(logical as f32);
        let adv = fb.measure_ttf_mono("M", px).max(1) as u32;
        Metrics { px, adv, lh: scale().px(logical + LEADING) }
    }
}
