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

//! Where each title sits and how wide it is, measured in the face it is
//! drawn in.

use super::items::{title, TITLE_COUNT};
use super::metrics::pad_x;
use crate::render::measure_aa::{measure_aa, measure_aa_bold};
use crate::render::topbar::brand_right;
use crate::render::ui_font::MENU_PX;
use crate::state::Context;

/// The focused app's title is set in the bold face, so it has to be measured
/// with that face or the hit box drifts off the glyphs.
pub(super) fn title_w(ctx: &Context, index: usize) -> u32 {
    let label = title(ctx, index);
    let w = if index == 0 { measure_aa_bold(label, MENU_PX) } else { measure_aa(label, MENU_PX) };
    w + pad_x() * 2
}

pub(super) fn title_x(ctx: &Context, index: usize) -> u32 {
    let mut x = brand_right();
    for i in 0..index.min(TITLE_COUNT) {
        x += title_w(ctx, i);
    }
    x
}

/// Where the last title ends: the tray's labels start right of it.
pub fn titles_right(ctx: &Context) -> u32 {
    title_x(ctx, TITLE_COUNT)
}
