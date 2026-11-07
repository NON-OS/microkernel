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
//! A buffer committed on a popup in a window of its own (window.rs): shown
//! on its own NONOS surface, at the window's ratio, a new surface for a new
//! shape taking the old one's place in the stack.

use alloc::string::String;

use nonos_app_skeleton::clients::compositor::damage_commit;
use nonos_app_skeleton::discover::{lookup_port, lookup_service};

use crate::linux::guest::Guest;

use super::super::present::layer::submit;
use super::super::present::scaled::popup;
use super::super::present::source::take;
use super::super::present::surface::{register, say};
use super::super::scene_pixels::Pixels;

/// `buffer` on `surface`, shown in its popup's window; false when the
/// surface is no popup with a window of its own.
pub fn present(guest: &mut Guest, surface: u32, buffer: u32) -> bool {
    let Some(at) = guest.scene.popup_windows.iter().position(|w| w.surface == surface) else {
        return false;
    };
    let src = match popup(&guest.scene, &guest.memfd_spans, surface, buffer) {
        Ok(src) => src,
        Err(why) => {
            say(String::from_utf8_lossy(why).into());
            return true;
        }
    };
    let (pid, comp) = (guest.pid, lookup_port(b"compositor"));
    let rid = guest.scene.next_serial();
    let w = &mut guest.scene.popup_windows[at];
    let (x, y) = w.at;
    let (sw, sh, _) = src.shape;
    if w.shape != Some(src.shape) {
        let Some(mut px) = Pixels::with_room(src.bytes) else { return true };
        let filled = px.frame(src.bytes).is_some_and(|out| take(pid, &src, out));
        if !filled {
            return true;
        }
        let compositor = lookup_service(b"compositor").map_or(0, |p| p.pid);
        let Ok((_, handle)) = register(&px, src.shape, (compositor, 0)) else { return true };
        let Some(comp) = comp else { return true };
        let replaces = w.out.unwrap_or(0);
        if let Err(why) = submit((comp, rid), handle, (x, y), (sw, sh), (w.window, replaces)) {
            let _ = nonos_libc::mk_surface_release(handle);
            say(alloc::format!("[WAYLAND] popup {sw}x{sh} not placed: {why}\n"));
            return true;
        }
        let old = (w.out.replace(handle), core::mem::replace(&mut w.pixels, px));
        w.shape = Some(src.shape);
        if let (Some(old), pixels) = old {
            if nonos_libc::mk_surface_release(old) < 0 {
                core::mem::forget(pixels);
            }
        }
    } else if !w.pixels.frame(src.bytes).is_some_and(|out| take(pid, &src, out)) {
        return true;
    }
    if let Some(port) = comp {
        let _ = damage_commit(port, guest.scene.next_serial(), x, y, sw, sh);
    }
    true
}
