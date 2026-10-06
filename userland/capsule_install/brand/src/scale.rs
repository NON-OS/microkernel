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

//! One scale for first-boot setup and the installer: how many pixels a
//! logical pixel takes on the canvas they draw on, the unit every space is a
//! multiple of, and the type sizes, all from one place.
//!
//! The scale goes by the canvas's short side: 1.25 from 1000, 1.5 from 1440
//! and 2 from 2160. The compositor hands a panel of 2560 by 1440 or more a
//! canvas half its size and doubles it onto the screen, so such a panel shows
//! setup as a panel of half its size would: a 4K laptop's 1920 by 1080 canvas
//! is drawn at 1.25, as a 1080p laptop is, and its text comes out the same
//! size on the glass. Type is drawn at its size, never stretched, so the
//! doubling costs no sharpness.

#[path = "scale_rule.rs"]
mod rule;

/// Logical pixels per spacing unit. Every margin, gap, row and band in setup
/// and the installer is a whole number of these.
pub const UNIT: u32 = 8;

/// The type sizes, a ramp from the body size by a ratio of 1.2 a step,
/// rounded to whole pixels: labels two steps down, captions one, headings one
/// up, the lead (a screen's one-line summary) two, titles four.
pub const LABEL_PX: f32 = 12.0;
pub const CAPTION_PX: f32 = 14.0;
pub const BODY_PX: f32 = 17.0;
pub const HEADING_PX: f32 = 20.0;
pub const LEAD_PX: f32 = 24.0;
pub const TITLE_PX: f32 = 35.0;

/// The ratio between neighbouring steps of the type ramp.
pub const TYPE_RATIO: f32 = 1.2;

/// Drawing pixels per logical pixel, in quarters: 4 is one to one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Scale(u32);

impl Scale {
    pub const ONE: Scale = Scale(4);

    /// The scale for a canvas `width` by `height`, from its short side.
    pub fn for_canvas(width: u32, height: u32) -> Scale {
        Scale(rule::quarters_for(width, height))
    }

    pub fn quarters(self) -> u32 {
        self.0
    }

    /// `v` logical pixels on the canvas, to the nearest pixel.
    pub fn px(self, v: u32) -> u32 {
        v.saturating_mul(self.0).saturating_add(2) / 4
    }

    /// `n` spacing units on the canvas.
    pub fn units(self, n: u32) -> u32 {
        self.px(UNIT.saturating_mul(n))
    }

    /// A type size on the canvas.
    pub fn font(self, px: f32) -> f32 {
        px * self.0 as f32 / 4.0
    }
}
