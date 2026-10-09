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

//! Where everything on the full-screen installer goes, from the canvas size
//! alone, in setup's arrangement: the brand panel on the left with the
//! steps, the screen in a column beside it.
//!
//! The screen's body started at y 190 whatever the canvas, under a title at
//! y 86, with 64 pixels kept for the keys. On the 1280 by 720 canvas a 2560
//! by 1440 panel gets, that left the body 450 pixels where the confirm and
//! proofs screens need 512, and the word to type ran under the keys. Now the
//! body is at least `MOST_BODY` units on every canvas: the title moves up on
//! a short canvas, and on the shortest is set a step down the type ramp.

use nonos_brand::scale::{Scale, BODY_PX, CAPTION_PX, LABEL_PX, LEAD_PX, TITLE_PX};

use super::super::metrics::{Metrics, MOST_BODY};

/// Rows in the panel: setup's, then the five steps.
pub const STEPS: u32 = 6;

const LABEL_BAND: u32 = 2;
const TITLE_BAND: u32 = 6;
const COMPACT_TITLE_BAND: u32 = 4;
const LINE: u32 = 3;
/// Marker, a unit, the title band, a unit, the subtitle line, two units.
const fn header(title_band: u32) -> u32 {
    LABEL_BAND + 1 + title_band + 1 + LINE + 2
}
/// A unit over the keys' rule, and the keys' band.
const FOOT_GAP: u32 = 1;
const FOOT_BAND: u32 = 6;
const MIN_TOP: u32 = 6;
const MAX_TOP: u32 = 16;
const COLUMN_MAX: u32 = 100;
const MARGIN: u32 = 8;

#[derive(Clone, Copy, Debug)]
pub struct FullLayout {
    pub scale: Scale,
    pub unit: u32,

    pub panel_w: u32,
    pub emblem_x: u32,
    pub emblem_y: u32,
    pub emblem_w: u32,
    pub caption_y: u32,
    pub release_y: u32,
    pub steps_x: u32,
    pub steps_y: u32,
    pub step_h: u32,

    pub col_x: u32,
    pub col_w: u32,
    pub marker_y: u32,
    pub title_y: u32,
    pub title_px: f32,
    pub sub_y: u32,
    /// The screen's own body.
    pub body_y: u32,
    pub body_h: u32,
    /// The rule over the keys, and the top of the keys' line.
    pub foot_y: u32,
    pub keys_y: u32,

    pub label_px: f32,
    pub caption_px: f32,
    pub body_px: f32,
    /// The sizes the screens draw at.
    pub m: Metrics,
}

pub fn layout(width: u32, height: u32) -> FullLayout {
    let scale = Scale::for_canvas(width, height);
    let m = Metrics::at(scale);
    let unit = m.unit.max(1);
    let u = |n: u32| n.saturating_mul(unit);
    let h_units = height / unit;
    let footer = FOOT_GAP + FOOT_BAND;

    let roomy = MIN_TOP + header(TITLE_BAND) + MOST_BODY + footer <= h_units;
    let (band, title_px) = if roomy {
        (TITLE_BAND, scale.font(TITLE_PX))
    } else {
        (COMPACT_TITLE_BAND, scale.font(LEAD_PX))
    };
    let spare = h_units.saturating_sub(header(band) + MOST_BODY + footer);
    let top = (h_units / 10).min(spare).clamp(MIN_TOP, MAX_TOP);

    let marker_y = u(top);
    let title_y = marker_y + u(LABEL_BAND + 1);
    let sub_y = title_y + u(band + 1);
    let body_y = marker_y + u(header(band));
    let foot_y = height.saturating_sub(u(FOOT_BAND));
    let body_h = foot_y.saturating_sub(u(FOOT_GAP)).saturating_sub(body_y);

    let panel_w = width * 36 / 100;
    let pane = width.saturating_sub(panel_w);
    let col_w = u(COLUMN_MAX).min(pane.saturating_sub(u(2 * MARGIN)));
    let col_x = panel_w + (pane.saturating_sub(col_w) / 2).max(u(MARGIN));

    // The panel's block, centred top to bottom: the emblem, the caption, the
    // release, then the six rows, four units each where that leaves six units
    // at each end, else three.
    let emblem_w = (panel_w * 2 / 5).clamp(u(12), u(27));
    let emblem_h = emblem_w * 10 / 9;
    let block = |step: u32| emblem_h + u(5 + LABEL_BAND + 1 + LABEL_BAND + 7) + STEPS * u(step);
    let step = if block(4) + u(2 * MIN_TOP) <= height { 4 } else { 3 };
    let emblem_y = height.saturating_sub(block(step)) / 2;
    let caption_y = emblem_y + emblem_h + u(5);
    let release_y = caption_y + u(LABEL_BAND + 1);
    let steps_y = release_y + u(LABEL_BAND + 7);

    FullLayout {
        scale,
        unit,
        panel_w,
        emblem_x: panel_w.saturating_sub(emblem_w) / 2,
        emblem_y,
        emblem_w,
        caption_y,
        release_y,
        steps_x: panel_w / 6,
        steps_y,
        step_h: u(step),
        col_x,
        col_w,
        marker_y,
        title_y,
        title_px,
        sub_y,
        body_y,
        body_h,
        foot_y,
        keys_y: foot_y + u((FOOT_BAND - LABEL_BAND) / 2),
        label_px: scale.font(LABEL_PX),
        caption_px: scale.font(CAPTION_PX),
        body_px: scale.font(BODY_PX),
        m,
    }
}
