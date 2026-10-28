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

//! A dialog's first buffer laid on the window, centred on it (dialog.rs).

use crate::linux::guest::Guest;

use super::super::input::enter::keyboard_on;
use super::super::present::scale_math::to_logical;
use super::stack::Popup;

/// A commit on `surface`: a dialog's first buffer lays it on the window.
pub fn commit(guest: &mut Guest, surface: u32) {
    let scene = &mut guest.scene;
    let Some(s) = scene.surfaces.iter().find(|s| s.id == surface) else { return };
    let (Some(t), Some(b)) = (s.toplevel, s.pending) else { return };
    let Some(b) = scene.buffers.iter().find(|x| x.id == b) else { return };
    let (w, h) = (b.width as i32, b.height as i32);
    let Some(shown) = scene.shown.filter(|w| w.toplevel != Some(t)) else { return };
    if !scene.popups.dialogs.contains(&t) {
        return;
    }
    if let Some(p) = scene.popups.list.iter_mut().find(|p| p.id == t) {
        (p.rect.2, p.rect.3) = (w, h);
        return;
    }
    let Some((_, _, ww, wh)) = scene.rect() else { return };
    let e = scene.frame.ratio;
    let (ww, wh) = (to_logical(ww as i32, e), to_logical(wh as i32, e));
    let at = (((ww - w) / 2).max(0), ((wh - h) / 2).max(0));
    let parent = shown.surface;
    scene.popups.list.push(Popup {
        id: t,
        surface,
        parent,
        rect: (at.0, at.1, w, h),
        grabbed: true,
    });
    super::layer::lay(scene, t, surface, at);
    if let Some(keyboard) = guest.scene.keyboard {
        let _ = keyboard_on(guest, keyboard);
    }
}
