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
//! the display's fill rect when the mode fills it: the window's whole, so a
//! maximised guest is told the content its frame leaves of it.

use nonos_app_skeleton::clients::compositor::display_info;
use nonos_app_skeleton::discover::lookup_port;
use nonos_app_skeleton::runner::chrome::full_screen;

use crate::linux::guest::Guest;

use super::super::configure::configure;
use super::super::fit::{Configure, Mode, Rect};
use super::super::present::scale_math::{display, expected_ratio, to_logical, Scale};
use super::super::present::scaled::quarters;
use super::super::present::surface::say;

pub(super) fn ask(guest: &mut Guest, toplevel: u32, want: Mode) {
    super::super::popup::ends::end_all(guest);
    let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.toplevel == Some(toplevel)) else {
        return;
    };
    // Asked before its first configure: told in that one (first.rs).
    if s.initial {
        s.wants = Some(want);
        return;
    }
    let Some(xdg) = s.xdg else { return };
    // A window behind the shown one has no place to fill until it is shown.
    let behind = |w: &super::super::scene_window::Window| {
        w.shown.is_some_and(|w| w.toplevel == Some(toplevel))
    };
    if guest.scene.behind.iter().any(behind) {
        return;
    }
    // Nor one not shown yet while another window is: the fit here is that
    // window's, and taking the ask on it filled the wrong window later. It
    // is kept, and asked once its own window is shown (held).
    let other = guest.scene.win.shown.is_some_and(|w| w.toplevel != Some(toplevel));
    if other {
        if let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.toplevel == Some(toplevel)) {
            s.wants = Some(want);
        }
        return;
    }
    let fill = match want.fills() {
        true => match fill() {
            Some(rect) if want.bare() => rect,
            Some(rect) => super::super::present::frame::now::inset(&guest.scene, rect),
            None => {
                say("[WAYLAND] full screen refused: the compositor gave no display size\n".into());
                return;
            }
        },
        false => (0, 0, 0, 0),
    };
    let now = guest.scene.rect();
    let serial = guest.scene.next_serial();
    if let Some(told) = guest.scene.win.fit.ask(want, now, fill, serial) {
        guest.scene.win.fit_of = Some(toplevel);
        let told = logical(told, ratio(guest, toplevel));
        let how = (guest.scene.win.focused, super::bounds::of(guest, toplevel));
        configure(toplevel, xdg, &told, how, &mut guest.display.to_client);
    }
}

/// The mode the toplevel on `surface` asked while its window was not shown
/// yet (ask), asked now that it is.
pub fn held(guest: &mut Guest, surface: u32) {
    let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == surface) else { return };
    if s.initial {
        return;
    }
    let (Some(want), Some(toplevel)) = (s.wants.take(), s.toplevel) else { return };
    ask(guest, toplevel, want);
}

/// The ratio `toplevel`'s window is shown at: the one its buffer was shown
/// at, or, asked before its first buffer (a maximised or full-screen start),
/// the one it will be shown at. The window's ratio is a default 1:1 until a
/// buffer is shown, so a Qt client asking full screen at its first commit on
/// a 1.25 display was told the screen in its pixels, drew a buffer a
/// quarter larger each way and was shown past the screen's edges.
fn ratio(guest: &Guest, toplevel: u32) -> u32 {
    let shown = guest.scene.win.shown.is_some_and(|w| w.toplevel == Some(toplevel));
    if shown {
        return guest.scene.win.frame.ratio;
    }
    let scale = guest.scene.surfaces.iter().find(|s| s.toplevel == Some(toplevel));
    expected_ratio(scale.map_or(Scale::default(), |s| s.scale), display(quarters()))
}

/// The green button's rect on this display: from the foot of the menubar
/// down to the bottom edge, the whole width.
pub(super) fn fill() -> Option<Rect> {
    let port = lookup_port(b"compositor")?;
    let d = display_info(port, 1).ok()?;
    Some(full_screen(d.width, d.height))
}

/// `told` with its size in the logical pixels the guest draws in, at the
/// window's ratio; where it goes on screen stays in the display's pixels.
pub(super) fn logical(told: Configure, e: u32) -> Configure {
    let size = |v: u32| to_logical(v as i32, e) as u32;
    Configure { rect: told.rect.map(|(x, y, w, h)| (x, y, size(w), size(h))), ..told }
}
