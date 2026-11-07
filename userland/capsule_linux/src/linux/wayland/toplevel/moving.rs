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

//! xdg_toplevel.move: a client that draws its own title bar (Telegram,
//! GTK's headerbar, Firefox's tab strip) asks for its window to be moved
//! from a press on it. Taken with the serial of the latest press while the
//! button is held, on the shown window in its own size: the window then
//! follows the pointer as when its NØNOS title bar is dragged
//! (present/frame/drag.rs, whose state this starts), until the button is let
//! go. The pointer leaves the client's surface, as the move holds it.

use nonos_app_skeleton::runner::chrome::menubar_for;

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::fit::Mode;

pub fn moving(guest: &mut Guest, toplevel: u32, args: &mut Args<'_>) {
    let (_seat, serial) = (args.u32(), args.u32());
    let scene = &guest.scene;
    let held = scene.dnd.pressed && serial.is_some() && scene.popups.last_press == serial;
    let ours = scene.win.shown.and_then(|w| w.toplevel) == Some(toplevel);
    let (Some(at), true, true) = (scene.win.at, held && ours, scene.win.fit.now() == Mode::Normal)
    else {
        return;
    };
    let (x, y) = scene.win.frame.pointer;
    let (w, h) = super::super::output::screen();
    let scene = &mut guest.scene;
    scene.win.frame.drag = Some((at, (at.0 as i32 + x, at.1 as i32 + y)));
    scene.win.frame.top = menubar_for(w, h);
    // The window moves: its popups end now, not at the window manager's
    // notice a frame or more later, drawn at their old place meanwhile.
    super::super::popup::ends::end_all(guest);
    super::super::input::off::pointer_off(guest);
}
