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

use nonos_app_skeleton::{feed, InputEvent, InputKind};
use nonos_toolkit::widgets::Step;

use crate::browser::omnibox::Change;
use crate::browser::state::State;

/* Pointer motion and a button coming up, fed to the settings page's
 * controls in content coordinates; off a page the window draws, or off the
 * content area, the pointer has left them. A change of what is lit or
 * pressed redraws the page. */
pub(super) fn settings_pointer(state: &mut State, event: InputEvent) {
    let at = super::local::local(event, state.layout.content).filter(|_| state.on_own);
    let c = &mut state.ui.settings;
    let step = match (event.kind, at) {
        (InputKind::ButtonUp, _) => c.released(),
        (_, Some(at)) => feed(c, &at),
        (_, None) => c.left(),
    };
    if step == Step::Repaint {
        state.mark(Change::Page);
    }
}
