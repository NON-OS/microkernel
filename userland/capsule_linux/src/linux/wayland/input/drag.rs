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

//! Pointer input while a drag holds the pointer (dnd/): its motion moves the
//! drag over the surface under it, a popup's before the window's, or over
//! nothing on the frame; the button let go ends it; other presses and the
//! wheel go nowhere. Keys go on to the client. Whether a button is held is
//! kept for every press, as a drag starts only during one.

use nonos_app_skeleton::input::{InputEvent, InputKind};

use crate::linux::guest::Guest;

use super::super::dnd;
use super::super::popup::corner::on_surface;
use super::super::present::frame::pointer as frame;
use super::deliver::logical;

/// True when the drag took `event`.
pub fn taken(guest: &mut Guest, event: &InputEvent) -> bool {
    use InputKind::*;
    if matches!(event.kind, ButtonDown | ButtonUp) {
        guest.scene.dnd.pressed = event.kind == ButtonDown;
        let_go_kept(guest);
    }
    if !guest.scene.dnd.dragging() {
        return false;
    }
    match event.kind {
        PointerAbs => {
            let on = frame::motion(guest, event.x, event.y).map(|p| logical(guest, p));
            let under = on.and_then(|(x, y)| match guest.scene.popups.under(x, y) {
                Some((popup, at)) => {
                    let (px, py) = on_surface(&guest.scene, popup, at);
                    Some((popup, px, py))
                }
                None => guest.scene.win.shown.map(|w| (w.surface, x, y)),
            });
            dnd::over::motion(guest, under);
            true
        }
        ButtonUp => {
            dnd::end::release(guest);
            true
        }
        ButtonDown | Wheel => true,
        _ => false,
    }
}

/// A press kept through the keyboard leaving the window names nothing once
/// its button is let go, or another is pressed, while the guest has no
/// keyboard: a grab with it then would open a menu over the window that
/// has the keyboard.
fn let_go_kept(guest: &mut Guest) {
    let scene = &mut guest.scene;
    if !core::mem::take(&mut scene.dnd.kept) {
        return;
    }
    let focused = scene.win.focused || scene.behind.iter().any(|w| w.focused);
    if !focused {
        scene.popups.forget_input();
    }
}
