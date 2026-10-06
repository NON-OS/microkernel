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

use nonos_app_skeleton::{EventOutcome, InputEvent};

use crate::browser::omnibox::{focus_after, search_bar_hit, Focus, Region};
use crate::browser::paint::home_page;
use crate::browser::state::{Origin, State};

/* A click on the home page, hit-tested at the width it was painted. The
 * search bar takes the keyboard; a shortcut goes to its site. */
pub fn on_home_click(state: &mut State, event: InputEvent) -> EventOutcome {
    let w = state.viewport_w;
    let region = if search_bar_hit(event.x, event.y, w) { Region::Omnibox } else { Region::Page };
    match focus_after(region, None, state.ui.kbd) {
        (Focus::Omnibox, _) if state.ui.kbd == Focus::Omnibox => {}
        (Focus::Omnibox, _) => {
            state.focus_omnibox();
            state.fit_text();
        }
        (Focus::Page, field) => state.focus_page(field),
    }
    match home_page::shortcut_url_at(event.x, event.y, w) {
        Some(url) => super::navigate::navigate(state, url.into(), Origin::User),
        None => EventOutcome::Idle,
    }
}
