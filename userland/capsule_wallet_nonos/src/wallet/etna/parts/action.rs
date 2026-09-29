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

//! The two weights of action, and no third. A primary is cyan with ink text;
//! a secondary is an outline in the text colour; an action that cannot be
//! pressed is an outline in text-3, never a faded cyan.

use nonos_app_skeleton::PaintBuffer;

use super::super::rect::Rect;
use super::super::roles::Role;
use super::super::text::{draw_in, line, width};
use super::super::tokens::{CORNER, CYAN, INK, OUTLINE, TEXT, TEXT_3};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Weight {
    Primary,
    Secondary,
}

pub fn action(fb: &mut PaintBuffer, at: Rect, title: &str, weight: Weight, enabled: bool) {
    let filled = weight == Weight::Primary && enabled;
    let ink = match (filled, enabled) {
        (true, _) => INK,
        (false, true) => TEXT,
        (false, false) => TEXT_3,
    };
    if filled {
        fb.fill_round(at.x, at.y, at.w, at.h, CORNER, CYAN);
    } else {
        fb.stroke_round(at.x, at.y, at.w, at.h, CORNER, 1, OUTLINE);
    }
    let tx = at.x as i32 + (at.w as i32 - width(Role::Button, title)) / 2;
    let ty = at.y as i32 + (at.h as i32 - line(Role::Button)) / 2;
    draw_in(fb, tx, ty, Role::Button, title, ink);
}
