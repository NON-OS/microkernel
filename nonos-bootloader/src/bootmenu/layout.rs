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

//! The menu's geometry: the shared scene (the brand frame left, the menu
//! right, the footer under a full rule), and the menu's rows inside it.

use super::about::ABOUT_LINES;
use super::entries::ENTRIES;
use crate::display::ink::{metrics, scene, Scene, Style};

#[derive(Clone, Copy)]
pub(super) struct Layout {
    pub s: Scene,
    pub u: u32,
    pub frame: (u32, u32, u32, u32),
    pub col_x: u32,
    pub col_w: u32,
    pub list_y: u32,
    pub row_h: u32,
    pub about_y: u32,
    pub platform_y: u32,
    pub footer_y: u32,
}

pub(super) fn layout() -> Layout {
    let (mono, body, label) = (metrics(Style::Mono), metrics(Style::Body), metrics(Style::Label));
    let u = scene(0).u;
    let row_h = label.line + 4 * u;
    let list_h = row_h * ENTRIES.len() as u32;
    let about_h = body.line * (ABOUT_LINES + 2) + mono.line + 6 * u;
    let s = scene(mono.line + 4 * u + list_h + 6 * u + about_h + 6 * u + mono.line * 2 + 2 * u);
    let list_y = s.col_y + mono.line + 4 * u;
    let about_y = list_y + list_h + 6 * u;
    Layout {
        s,
        u,
        frame: s.frame,
        col_x: s.col_x,
        col_w: s.col_w,
        list_y,
        row_h,
        about_y,
        platform_y: about_y + about_h + 6 * u,
        footer_y: s.footer_y,
    }
}
