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

//! A press outside every grabbing popup, on the window or its frame, ends
//! the grabs, as a compositor dismisses a menu: each popup is told
//! popup_done, topmost first. A press on the window's own content then
//! reaches the client, as mutter and wlroots deliver it, so a click on
//! another menu in a menubar opens it at once; one on the NØNOS frame or a
//! tooltip, and its release, reach nothing.

use nonos_app_skeleton::input::{InputEvent, InputKind};

use crate::linux::guest::Guest;

/// True when the press or release was taken by a dismissal.
pub fn taken(guest: &mut Guest, event: &InputEvent) -> bool {
    match event.kind {
        InputKind::ButtonDown => {
            let popups = &guest.scene.popups;
            let on = guest.scene.pointer_on;
            let grabbing = popups.top().is_some();
            let inside = popups.list.iter().any(|p| p.grabbed && Some(p.surface) == on);
            if !grabbing || inside {
                return false;
            }
            let content = on.is_some() && on == guest.scene.win.shown.map(|w| w.surface);
            super::super::popup::ends::end_all(guest);
            guest.scene.popups.dismissed = !content;
            !content
        }
        InputKind::ButtonUp => core::mem::take(&mut guest.scene.popups.dismissed),
        _ => false,
    }
}
