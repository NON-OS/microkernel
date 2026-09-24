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

//! A bordered panel with a small caption on its top edge.

use nonos_app_skeleton::PaintBuffer;

use crate::install::ui::metrics::{RADIUS, SMALL_PX};
use crate::install::ui::{text, theme};

/// Paints the card and returns the y where content starts inside it.
pub fn card(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, caption: &str) -> u32 {
    fb.panel(x, y, w, h, RADIUS, theme::CARD_BG, theme::CARD_BORDER);
    if !caption.is_empty() {
        text::line(fb, x + 16, y + 10, caption, theme::MUTED, SMALL_PX);
        return y + 36;
    }
    y + 16
}
