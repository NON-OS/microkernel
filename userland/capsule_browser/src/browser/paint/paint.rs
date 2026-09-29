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

use crate::browser::omnibox::Damage;
use crate::browser::paint::{chrome, home_page, page_parts};
use crate::browser::state::{State, View};

/* Draw what `parts` says changed. A full paint draws the chrome, the view
 * and the settings panel over it; otherwise each dirtied part redraws on
 * its own: the pill for a keystroke, the toolbar for a load starting or
 * ending, the page, a scroll, or just the hovered-link bubble. */
pub fn paint(state: &mut State, fb: &mut PaintBuffer, parts: Damage) {
    if parts.has(Damage::FULL) {
        chrome::paint(state, fb);
        match state.view {
            View::Home => home_page::paint(state, fb),
            View::Page => page_parts::page_full(state, fb),
        }
        crate::browser::settings::paint(state, fb);
        return;
    }
    if parts.has(Damage::TOOLBAR) {
        chrome::paint(state, fb);
    } else if parts.has(Damage::PILL) {
        chrome::pill(state, fb);
    }
    match state.view {
        View::Home if parts.has(Damage::PAGE) => home_page::paint(state, fb),
        View::Home if parts.has(Damage::HOME_BAR) => home_page::search_bar(state, fb),
        View::Home => {}
        View::Page => page_parts::page_parts(state, fb, parts),
    }
}
