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

//! A mode asked for: the configure that tells the guest its new size, with
//! the display's fill rect when the mode fills it.

use nonos_app_skeleton::clients::compositor::display_info;
use nonos_app_skeleton::discover::lookup_port;
use nonos_app_skeleton::runner::chrome::full_screen;

use crate::linux::guest::Guest;

use super::super::configure::configure;
use super::super::fit::{Mode, Rect};
use super::super::present::surface::say;

pub(super) fn ask(guest: &mut Guest, toplevel: u32, want: Mode) {
    super::super::popup::ends::end_all(guest);
    let Some(xdg) = guest.scene.surfaces.iter().find(|s| s.toplevel == Some(toplevel)).and_then(|s| s.xdg)
    else {
        return;
    };
    let fill = match want.fills() {
        true => match fill() {
            Some(rect) => rect,
            None => {
                say("[WAYLAND] full screen refused: the compositor gave no display size\n".into());
                return;
            }
        },
        false => (0, 0, 0, 0),
    };
    let now = guest.scene.rect();
    let serial = guest.scene.next_serial();
    if let Some(told) = guest.scene.fit.ask(want, now, fill, serial) {
        configure(toplevel, xdg, &told, &mut guest.display.to_client);
    }
}

/// The green button's rect on this display: from the foot of the menubar
/// down to the bottom edge, the whole width.
fn fill() -> Option<Rect> {
    let port = lookup_port(b"compositor")?;
    let d = display_info(port, 1).ok()?;
    Some(full_screen(d.width, d.height))
}
