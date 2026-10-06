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

//! Where everything on a setup screen goes, worked out from the canvas size
//! alone, so it can be checked for every display without drawing anything.
//!
//! Setup drew at fixed pixel positions laid out for 1440 by 900: content from
//! y 110 in 30 pixel rows, a 420 pixel list, a title at y 34. A 1920 by 1080
//! canvas left the bottom half of every screen empty, and the compositor
//! hands a 2560 by 1440 panel a 1280 by 720 canvas, where the tallest screens
//! came close to the footer. Now every space is a whole number of the brand's
//! unit, the type comes from its one ramp, both scaled by the canvas
//! (nonos_brand::scale), and the room left over goes to the screen: list rows
//! are five units where the tallest screen has room for them and four where
//! it does not, the title sits lower on a taller canvas, and the column is
//! centred in the space beside the panel at a width text reads at.

use nonos_brand::scale::{Scale, BODY_PX, CAPTION_PX, LABEL_PX, TITLE_PX};

/// The steps listed in the panel.
pub const STEPS: u32 = 13;

/// The most any screen puts under its title: list rows, lines of body text,
/// and gaps between those blocks. The Apps step with every optional app
/// present has the most (8 rows; "Required", its 3 lines and the 3 lines on
/// what off means; 2 gaps). Every other screen needs less; the layout proofs
/// check them screen by screen.
pub const MOST_ROWS: u32 = 8;
pub const MOST_LINES: u32 = 7;
pub const MOST_GAPS: u32 = 2;

/// Columns inside a row, in units from its left edge: an app's purpose beside
/// "off  " and its name; an answer beside its name on the review; a Qwen
/// tier's marker, name, size and fit.
pub const APPS_PURPOSE_AT: u32 = 28;
pub const REVIEW_VALUE_AT: u32 = 15;
pub const QWEN_CELLS: [u32; 3] = [3, 25, 35];
/// Lines under the Qwen list at most: the line saying which rows show, the
/// one saying how many tiers need more memory, the two on memory, and the
/// four on where the model comes from.
pub const QWEN_AFTER_LINES: u32 = 8;
/// A wallpaper's row on the Appearance step: whether it is kept, its name,
/// and whether it is the desktop's.
pub const WALL_CELLS: [u32; 3] = [3, 11, 36];
/// Lines under the wallpaper list at most: which rows show, how many are
/// kept, and the two on what keeping means.
pub const WALL_AFTER_LINES: u32 = 4;

/// Spaces in units. A label band holds one line of mono capitals, a title
/// band one title line, a body line one line of body text.
const LABEL_BAND: u32 = 2;
const TITLE_BAND: u32 = 6;
pub const LINE: u32 = 3;
pub const GAP: u32 = 2;
const FOOT_BAND: u32 = 8;
/// From the top of the step marker to the top of the screen's own content:
/// marker, a unit, the title, a unit, the subtitle line, three units.
const HEADER: u32 = LABEL_BAND + 1 + TITLE_BAND + 1 + LINE + 3;
/// From the bottom of the content to the bottom of the canvas.
const FOOTER: u32 = GAP + FOOT_BAND;
const MIN_TOP: u32 = 6;
const MAX_TOP: u32 = 16;
/// The widest the content column grows, and a list in it.
const COLUMN_MAX: u32 = 100;
const LIST_MAX: u32 = 70;
const MARGIN: u32 = 8;

#[derive(Clone, Copy, Debug)]
pub struct Layout {
    pub scale: Scale,
    pub width: u32,
    pub height: u32,
    /// One spacing unit on this canvas.
    pub unit: u32,

    /// The brand panel on the left, and the rule at its right edge.
    pub panel_w: u32,
    pub emblem_x: u32,
    pub emblem_y: u32,
    pub emblem_w: u32,
    pub caption_y: u32,
    pub release_y: u32,
    pub steps_x: u32,
    pub steps_y: u32,
    pub step_h: u32,

    /// The screen: a column beside the panel, `column` wide.
    pub col_x: u32,
    pub marker_y: u32,
    pub title_y: u32,
    pub sub_y: u32,
    /// The screen's own content runs from here to `body_end`.
    pub body_y: u32,
    pub body_end: u32,
    pub row_h: u32,
    pub line_h: u32,
    pub gap: u32,
    pub list_w: u32,
    /// The rule over the keys, and the top of the keys' line.
    pub foot_y: u32,
    pub keys_y: u32,

    pub label_px: f32,
    pub caption_px: f32,
    pub body_px: f32,
    pub title_px: f32,
}

/// Units the tallest screen's content takes with list rows `row` units high.
pub fn most_body(row: u32) -> u32 {
    MOST_ROWS * row + MOST_LINES * LINE + MOST_GAPS * GAP
}

/// The content column beside the panel, as (left edge, width): as wide as
/// text reads well, centred in the space right of the panel, with at least a
/// margin of eight units on either side.
pub fn column(width: u32, unit: u32) -> (u32, u32) {
    let panel_w = width * 36 / 100;
    let pane = width.saturating_sub(panel_w);
    let col_w = (COLUMN_MAX * unit).min(pane.saturating_sub(2 * MARGIN * unit));
    let col_x = panel_w + (pane.saturating_sub(col_w) / 2).max(MARGIN * unit);
    (col_x, col_w)
}

/// The run of a list's `rows` to show where `room` fit, keeping row `sel`
/// in it, as (first row, rows shown). A list that fits is shown whole.
pub fn window(rows: u32, room: u32, sel: u32) -> (u32, u32) {
    let shown = rows.min(room.max(1));
    let sel = sel.min(rows.saturating_sub(1));
    let first = sel.saturating_sub(shown / 2).min(rows - shown);
    (first, shown)
}

impl Layout {
    /// How many rows one body line high fit in the screen's content when
    /// `lines` body lines and `gaps` gaps follow them.
    pub fn line_rows(&self, lines: u32, gaps: u32) -> u32 {
        let room = self.body_end.saturating_sub(self.body_y);
        let after = lines.saturating_mul(self.line_h).saturating_add(gaps.saturating_mul(self.gap));
        room.saturating_sub(after) / self.line_h.max(1)
    }
}

pub fn layout(width: u32, height: u32) -> Layout {
    let scale = Scale::for_canvas(width, height);
    let unit = scale.units(1).max(1);
    let u = |n: u32| n.saturating_mul(unit);
    let h_units = height / unit;

    // Five unit rows where the tallest screen still fits under the smallest
    // top margin, else four.
    let fits = |row: u32| MIN_TOP + HEADER + most_body(row) + FOOTER <= h_units;
    let row = if fits(5) { 5 } else { 4 };
    let spare = h_units.saturating_sub(HEADER + most_body(row) + FOOTER);
    let top = (h_units / 10).min(spare).clamp(MIN_TOP, MAX_TOP);

    let marker_y = u(top);
    let title_y = marker_y + u(LABEL_BAND + 1);
    let sub_y = title_y + u(TITLE_BAND + 1);
    let body_y = marker_y + u(HEADER);
    let foot_y = height.saturating_sub(u(FOOT_BAND));
    let body_end = foot_y.saturating_sub(u(GAP));

    let panel_w = width * 36 / 100;
    let (col_x, col_w) = column(width, unit);
    let list_w = col_w.min(u(LIST_MAX));

    // The panel's block, centred top to bottom: the emblem, the caption and
    // the release under it, then the steps, four units a row where that fits
    // the canvas with six units to spare at each end, else three.
    let emblem_w = (panel_w * 2 / 7).clamp(u(9), u(23));
    let emblem_h = emblem_w * 10 / 9;
    let block = |step: u32| emblem_h + u(4 + LABEL_BAND + 1 + LABEL_BAND + 6) + STEPS * u(step);
    let step = if block(4) + u(2 * MIN_TOP) <= height { 4 } else { 3 };
    let emblem_y = height.saturating_sub(block(step)) / 2;
    let caption_y = emblem_y + emblem_h + u(4);
    let release_y = caption_y + u(LABEL_BAND + 1);
    let steps_y = release_y + u(LABEL_BAND + 6);

    Layout {
        scale,
        width,
        height,
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
        marker_y,
        title_y,
        sub_y,
        body_y,
        body_end,
        row_h: u(row),
        line_h: u(LINE),
        gap: u(GAP),
        list_w,
        foot_y,
        keys_y: foot_y + u((FOOT_BAND - LABEL_BAND) / 2),
        label_px: scale.font(LABEL_PX),
        caption_px: scale.font(CAPTION_PX),
        body_px: scale.font(BODY_PX),
        title_px: scale.font(TITLE_PX),
    }
}
