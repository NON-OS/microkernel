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

use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::widgets::rect::{centre, contains};
use nonos_toolkit::widgets::Rect;

use super::metrics::metrics;
use super::rect::{self, at};
use super::setup_geom_rows::TOGGLES;

pub const SECTIONS: usize = 3;

pub fn head_h() -> u32 {
    line_height(metrics().px_head).max(1) as u32
}

pub fn band_h(section: usize) -> u32 {
    let m = metrics();
    match section {
        2 => m.row_h * TOGGLES as u32,
        _ => m.chip_h,
    }
}

fn section_h(section: usize) -> u32 {
    head_h() + metrics().gap_tight + band_h(section)
}

fn panel_h() -> u32 {
    let sections: u32 = (0..SECTIONS).map(section_h).sum();
    let m = metrics();
    sections + m.gap_wide * SECTIONS as u32 + m.btn_h
}

pub fn panel(w: u32, h: u32) -> Rect {
    let m = metrics();
    centre(rect::content(w, h), m.btn_w * 2 + m.gap, panel_h())
}

fn section_top(w: u32, h: u32, section: usize) -> u32 {
    let gap = metrics().gap_wide;
    let prior: u32 = (0..section.min(SECTIONS)).map(|s| section_h(s) + gap).sum();
    panel(w, h).y + prior
}

pub fn head(w: u32, h: u32, section: usize) -> Rect {
    let p = panel(w, h);
    at(p.x, section_top(w, h, section), p.w, head_h())
}

pub fn band(w: u32, h: u32, section: usize) -> Rect {
    let p = panel(w, h);
    let top = section_top(w, h, section) + head_h() + metrics().gap_tight;
    at(p.x, top, p.w, band_h(section))
}

pub fn start(w: u32, h: u32) -> Rect {
    let (p, m) = (panel(w, h), metrics());
    let x = p.x + p.w.saturating_sub(m.btn_w) / 2;
    at(x, p.y + p.h.saturating_sub(m.btn_h), m.btn_w, m.btn_h)
}

pub fn start_at(w: u32, h: u32, x: i32, y: i32) -> bool {
    contains(start(w, h), x, y)
}
