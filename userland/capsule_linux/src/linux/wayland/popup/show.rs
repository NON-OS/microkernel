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

//! A popup that holds no grab, a tooltip or a completion list: configured at
//! its initial commit, as xdg-shell has every popup configured, and laid on
//! its window as a grabbing one is (layer.rs). The keyboard stays where it
//! is. It ends when the client destroys it, its parent goes, or the window's
//! grabs end.

use crate::linux::guest::Guest;

use super::super::subsurface::compose::repaint;

/// A commit on `surface`: shown, when it is a popup's not yet shown.
pub fn first(guest: &mut Guest, surface: u32) {
    let list = &mut guest.scene.popups.list;
    let Some(p) = list.iter_mut().find(|p| p.surface == surface && !p.shown) else { return };
    p.shown = true;
    let (id, rect) = (p.id, p.rect);
    if !super::window::open(guest, id, false) {
        super::layer::lay(&mut guest.scene, id, surface, (rect.0, rect.1));
    }
    super::tell::configure(guest, id);
    repaint(guest);
}
