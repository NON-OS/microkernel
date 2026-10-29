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

//! Sizes, in pixels at the desktop's base text size. Text grows with the
//! toolkit's text scale inside `text_ttf`; the boxes around it grow by the
//! same factor here, so a larger setting keeps every row's proportions.

use nonos_toolkit::ttf::text_scale::{scale, ONE};

pub const LIST_W: u32 = 300;
pub const ROW_H: u32 = 62;
pub const HEADER_H: u32 = 54;
pub const COMPOSE_H: u32 = 46;
pub const PAD: u32 = 12;
pub const GAP: u32 = 8;
pub const RADIUS: u32 = 12;
pub const LINE: u32 = 20;
pub const SMALL_LINE: u32 = 16;

pub const TITLE_PX: f32 = 17.0;
pub const NAME_PX: f32 = 15.0;
pub const BODY_PX: f32 = 14.0;
pub const SMALL_PX: f32 = 11.5;

#[derive(Clone, Copy)]
pub struct Metrics {
    factor: u32,
}

impl Metrics {
    pub fn now() -> Self {
        Metrics { factor: scale() }
    }

    /// `v` base pixels at the current scale.
    pub fn px(&self, v: u32) -> u32 {
        v * self.factor / ONE
    }

    /// The list's width in a window `width` wide: at most two fifths of it.
    pub fn list_w(&self, width: u32) -> u32 {
        self.px(LIST_W).min(width * 2 / 5)
    }
}
