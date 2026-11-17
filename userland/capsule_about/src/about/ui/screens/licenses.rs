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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::Rect;

use crate::about::state::State;

use super::super::metrics::metrics;
use super::{licenses_banner, licenses_table, licenses_text};

// Each card's top is the offset the painter places it at and the offset the extent
// is summed from, so the scrollbar cannot drift from the content: adding a card is
// one more step here, consumed by both.
fn table_top() -> u32 {
    licenses_banner::height() + metrics().card_gap
}

fn text_top() -> u32 {
    table_top() + licenses_table::height() + metrics().card_gap
}

// The one section whose content is genuinely unbounded: the extent is the banner
// plus the component table plus a row per licence line, and the scroll clamp in
// the frame depends on that count being the same one the painter walks.
pub fn content_h(_rect: &Rect) -> u32 {
    text_top() + licenses_text::height()
}

pub fn paint(state: &State, fb: &mut PaintBuffer, rect: &Rect) {
    let mut pane = fb.sub(rect.x, rect.y, rect.w, rect.h);
    let y = -(state.scroll as i32);
    licenses_banner::paint(&mut pane, y, rect.w);
    licenses_table::paint(&mut pane, y + table_top() as i32, rect.w);
    licenses_text::paint(&mut pane, y + text_top() as i32, rect.w);
}
