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

//! Sizes, from the brand's one scale (nonos_brand::scale): every space a
//! whole number of its unit, every type size a step of its ramp. A window is
//! drawn at one to one; the full screen at whatever the canvas asks for, so
//! the screens are written once against `Metrics` and look the same in both.
//!
//! The window is dialog shaped and opens centred. It used to be 640 high,
//! which the frame's title bar and shadow leave 579 of to draw in: the
//! confirm screen ran its word field under the footer, the proofs screen its
//! last row. It is as tall now as the tallest screen needs, `MOST_BODY` under
//! the header and over the footer; a display too short for that gets it
//! shrunk to fit by the app skeleton, as before.

use nonos_brand::scale::{Scale, BODY_PX, CAPTION_PX, HEADING_PX, LABEL_PX, LEAD_PX};

pub const WIN_W: u32 = 760;
pub const WIN_H: u32 = 704;
pub const WIN_X: u32 = 340;
pub const WIN_Y: u32 = 98;

/// The most any screen puts in its body, in units: the proofs screen with a
/// four line introduction (4 lines, a gap, 5 rows of 10) and the confirm
/// screen with all nine rows of the plan both come to 64.
pub const MOST_BODY: u32 = 64;

#[derive(Clone, Copy, Debug)]
pub struct Metrics {
    pub scale: Scale,
    pub unit: u32,
    /// The window's header and footer bands, and its padding.
    pub header_h: u32,
    pub footer_h: u32,
    pub pad: u32,
    /// One line of body text, a disk row, a progress bar's height, a corner.
    pub line_h: u32,
    pub row_h: u32,
    pub bar_h: u32,
    pub radius: u32,
    /// Space between blocks of a screen.
    pub gap: u32,
    /// A card: from its top to its first line, and under its last.
    pub card_top: u32,
    pub card_bottom: u32,
    /// Inset of a card's content, and of a row's text.
    pub inset: u32,
    /// A label's column in a key and value row.
    pub label_w: u32,
    /// The typed word's field.
    pub field_w: u32,
    pub field_h: u32,

    pub title_px: f32,
    pub lead_px: f32,
    pub body_px: f32,
    pub small_px: f32,
    pub label_px: f32,
    pub mono_px: f32,
    pub mono_small_px: f32,
}

impl Metrics {
    pub fn at(scale: Scale) -> Metrics {
        let u = |n: u32| scale.units(n);
        Metrics {
            scale,
            unit: u(1),
            header_h: u(8),
            footer_h: u(5),
            pad: u(3),
            line_h: u(3),
            row_h: u(7),
            bar_h: u(2),
            radius: u(1),
            gap: u(2),
            card_top: u(5),
            card_bottom: u(1),
            inset: u(2),
            label_w: u(24),
            field_w: u(36),
            field_h: u(5),
            title_px: scale.font(HEADING_PX),
            lead_px: scale.font(LEAD_PX),
            body_px: scale.font(BODY_PX),
            small_px: scale.font(CAPTION_PX),
            label_px: scale.font(LABEL_PX),
            mono_px: scale.font(BODY_PX),
            mono_small_px: scale.font(CAPTION_PX),
        }
    }

    /// The window's, at one to one.
    pub fn window() -> Metrics {
        Metrics::at(Scale::ONE)
    }

    /// A card holding `lines` lines.
    pub fn card_h(&self, lines: u32) -> u32 {
        self.card_top + lines * self.line_h + self.card_bottom
    }

    /// One proof on the proofs screen: its rule, a unit, three lines.
    pub fn proof_row_h(&self) -> u32 {
        self.unit + 3 * self.line_h
    }

    /// A window's body inside its content area `w` by `h`, as (x, y, w, h):
    /// under the header, over the footer, inside the padding.
    pub fn window_body(&self, w: u32, h: u32) -> (u32, u32, u32, u32) {
        (
            self.pad,
            self.header_h + self.pad / 2,
            w.saturating_sub(2 * self.pad),
            h.saturating_sub(self.header_h + self.footer_h + self.pad),
        )
    }
}

/// Where the confirm screen's blocks start below the top of its body, for a
/// plan of `plan_rows` rows: the plan's card, the line asking for the word,
/// the word's field, and the bottom of it all.
pub fn confirm_stack(m: &Metrics, plan_rows: u32) -> [u32; 4] {
    let plan = m.card_h(4) + m.gap;
    let ask = plan + m.card_h(plan_rows) + m.gap;
    let field = ask + m.line_h + m.unit;
    [plan, ask, field, field + m.field_h]
}
