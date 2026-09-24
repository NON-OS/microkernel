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

//! The head band and the source tabs.

use nonos_app_skeleton::PaintBuffer;

use crate::store::state::{State, TABS};
use crate::store::theme::{MUTED, TAB_BG_ACTIVE, TAB_FG, TAB_FG_ACTIVE, TITLE};

use super::counter::listed;
use super::metrics::{
    BODY_PX, HEAD_H, PAD_TOP, PAD_X, SMALL_PX, TAB_GAP, TAB_H, TAB_PAD_X, TITLE_PX,
};
use super::searchbar;
use super::text;

pub fn head(fb: &mut PaintBuffer, state: &State) {
    let top = text::top_of(PAD_TOP as i32, HEAD_H, TITLE_PX);
    text::line(fb, PAD_X, top, b"Marketplace", TITLE, TITLE_PX);
    let right = state.fb_w.saturating_sub(PAD_X);
    let field = searchbar::paint(fb, &state.search, right);
    let meta_top = text::top_of(PAD_TOP as i32, HEAD_H, BODY_PX);
    let count = state.visible().len();
    let at = right.saturating_sub(field + if field == 0 { 0 } else { PAD_X });
    text::right(fb, at, meta_top, &listed(count), MUTED, BODY_PX);
}

/// Where each tab sits.
pub fn tab_rect(index: usize) -> (u32, u32) {
    let mut x = PAD_X;
    for tab in TABS.iter().take(index) {
        x += text::width_of(tab.label(), SMALL_PX) + TAB_PAD_X * 2 + TAB_GAP;
    }
    let w = text::width_of(TABS[index].label(), SMALL_PX) + TAB_PAD_X * 2;
    (x, w)
}

pub fn tabs(fb: &mut PaintBuffer, state: &State) {
    let y = PAD_TOP + HEAD_H;
    for (i, tab) in TABS.iter().enumerate() {
        let (x, w) = tab_rect(i);
        let active = *tab == state.tab;
        if active {
            fb.fill_rect(x, y, w, TAB_H, TAB_BG_ACTIVE);
        }
        let fg = if active { TAB_FG_ACTIVE } else { TAB_FG };
        let top = text::top_of(y as i32, TAB_H, SMALL_PX);
        text::line(fb, x + TAB_PAD_X, top, tab.label(), fg, SMALL_PX);
    }
}
