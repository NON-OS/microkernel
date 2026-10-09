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

//! Where the body and its cells fall in a window of a given size and zoom,
//! for the painter and for sizing the screen before painting.

use nonos_app_skeleton::PaintBuffer;

use super::constants::{BODY_PAD_TOP, HEADER_H, TEXT_LEFT};
use super::footer::footer_h;
use super::metrics::Metrics;
use crate::layout::limits::LEFT_RAIL_W;
use crate::layout::{compute, Chrome, Layout, Rails};

pub fn geometry(fb: &PaintBuffer, font_scale: u32, rail_open: bool) -> (Layout, Metrics, Chrome) {
    let m = Metrics::new(fb, font_scale);
    let chrome = Chrome {
        titlebar_h: HEADER_H,
        tabstrip_h: 0,
        body_pad_top: BODY_PAD_TOP,
        footer_h: footer_h(),
        text_left: TEXT_LEFT,
        row_h: m.lh,
    };
    /*
     * The rail is off unless it was asked for. A terminal that opens with a
     * quarter of the window given to charts is a dashboard that happens to
     * accept commands; the grid is the window, and the telemetry is there for
     * whoever wants it.
     */
    let left = if rail_open { LEFT_RAIL_W } else { 0 };
    (compute(fb.width, fb.height, &chrome, Rails { left }), m, chrome)
}

/// The cells the body holds at this window size and zoom: columns between
/// the text margins, rows over the body and the prompt row together, which
/// a program in the foreground draws on.
pub fn grid_size(fb: &PaintBuffer, font_scale: u32, rail_open: bool) -> (usize, usize, Metrics) {
    let (l, m, chrome) = geometry(fb, font_scale, rail_open);
    let width = l.body.w.saturating_sub(2 * chrome.text_left);
    let cols = (width / m.adv.max(1)) as usize;
    let rows = ((l.body.h + l.input.h) / m.lh.max(1)) as usize;
    (cols.max(2), rows.max(1), m)
}
