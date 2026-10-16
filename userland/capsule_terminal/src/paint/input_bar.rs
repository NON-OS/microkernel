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

//! The inset panel the input line is drawn on.

use nonos_app_skeleton::PaintBuffer;

use crate::layout::Rect;
use crate::paint::constants::TEXT_LEFT;
use crate::paint::metrics::Metrics;
use crate::paint::shade::elevate;
use crate::term::theme::types::Theme;

// Warp-style input bar: an inset panel behind the prompt with a left
// accent stripe, drawn first so the prompt and text land on top.
pub(super) fn draw_bar(fb: &mut PaintBuffer, r: Rect, m: Metrics, t: &Theme) {
    let bar_x = r.x + TEXT_LEFT / 2;
    let bar_w = r.w.saturating_sub(TEXT_LEFT);
    let bar_y = r.y.saturating_sub(3);
    fb.fill_rect(bar_x, bar_y, bar_w, m.lh + 4, elevate(t.bg, 12));
    fb.fill_rect(bar_x, bar_y, 2, m.lh + 4, t.accent);
}
