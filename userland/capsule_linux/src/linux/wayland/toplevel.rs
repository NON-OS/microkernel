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

//! `xdg_toplevel`'s full screen and maximise, asked by the guest or by the
//! person with F11, and its destroy.
//!
//! A guest window has no NONOS frame and no green button: its pixels are
//! all the guest's. F11 is the key a Linux desktop makes a window full
//! screen with, so it is the person's way here. It is taken while the
//! focused guest has a toplevel, and goes to the guest as before when it
//! has none.

use nonos_app_skeleton::clients::compositor::display_info;
use nonos_app_skeleton::discover::lookup_port;
use nonos_app_skeleton::runner::chrome::full_screen;

use crate::linux::guest::Guest;

use super::args::Args;
use super::close_window::close_for;
use super::configure::configure;
use super::fit::{Mode, Rect};
use super::object::Object;
use super::present_surface::say;
use super::window_life::End;

/// set_maximized and set_fullscreen.
pub fn set(guest: &mut Guest, toplevel: u32, want: Mode, args: &mut Args<'_>) {
    // set_fullscreen names an output, or null; there is one display.
    if want == Mode::FullScreen {
        let _ = args.u32();
    }
    ask(guest, toplevel, want);
}

/// unset_maximized and unset_fullscreen: the saved rect back, when the
/// window is on its way to the mode being unset.
pub fn unset(guest: &mut Guest, toplevel: u32, which: Mode) {
    if guest.scene.fit.heading() == which {
        ask(guest, toplevel, Mode::Normal);
    }
}

/// F11: full screen, or back. False when the guest has no toplevel, so the
/// key is the guest's.
pub fn toggle(guest: &mut Guest) -> bool {
    let Some(toplevel) = shown_toplevel(guest) else { return false };
    let want = match guest.scene.fit.heading() {
        Mode::Normal => Mode::FullScreen,
        _ => Mode::Normal,
    };
    ask(guest, toplevel, want);
    true
}

/// xdg_toplevel.destroy: the window it made goes with it.
pub fn destroy(guest: &mut Guest, toplevel: u32) {
    close_for(&mut guest.scene, End::Toplevel(toplevel));
    for s in guest.scene.surfaces.iter_mut().filter(|s| s.toplevel == Some(toplevel)) {
        s.toplevel = None;
    }
    super::forget::forget(guest, Some(Object::XdgToplevel), toplevel);
}

/// Whether F11 is taken: the guest has a toplevel for it to act on.
pub fn has_toplevel(guest: &Guest) -> bool {
    shown_toplevel(guest).is_some()
}

/// The toplevel of the window shown, else of the first surface that has one.
fn shown_toplevel(guest: &Guest) -> Option<u32> {
    let scene = &guest.scene;
    scene.shown.and_then(|s| s.toplevel).or_else(|| scene.surfaces.iter().find_map(|s| s.toplevel))
}

fn ask(guest: &mut Guest, toplevel: u32, want: Mode) {
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
