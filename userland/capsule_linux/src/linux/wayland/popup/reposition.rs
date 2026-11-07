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

//! xdg_popup.reposition (xdg-shell 3): a popup placed again by a new
//! positioner, as a menu whose content grew is, or a completion list that
//! follows the text. It is told repositioned with the client's token, then
//! configured where it now is; a popup not yet shown takes the new place
//! and is told it with its first configure.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::ops::shell::POPUP_REPOSITIONED;
use super::super::out::Event;
use super::super::subsurface::compose::repaint;

pub fn reposition(guest: &mut Guest, id: u32, args: &mut Args<'_>) {
    let (Some(positioner), Some(token)) = (args.u32(), args.u32()) else { return };
    let popups = &guest.scene.popups;
    let Some(rule) = popups.positioners.iter().find(|(p, _)| *p == positioner).map(|r| r.1) else {
        return;
    };
    let Some(p) = popups.list.iter().find(|p| p.id == id) else { return };
    let (surface, parent, shown) = (p.surface, p.parent, p.shown);
    // A positioner xdg-shell calls incomplete leaves it where it was.
    let Some(rect) = super::made::rect(&guest.scene, parent, &rule) else { return };
    let mut grabbed = false;
    if let Some(p) = guest.scene.popups.list.iter_mut().find(|p| p.id == id) {
        (p.rect, p.rule, grabbed) = (rect, rule, p.grabbed);
    }
    if !shown {
        return;
    }
    // In a window of its own: placed again by the window manager.
    super::window::close(&mut guest.scene, id);
    if !super::window::open(guest, id, grabbed) {
        super::layer::lay(&mut guest.scene, id, surface, (rect.0, rect.1));
    }
    Event::new(id, POPUP_REPOSITIONED).u32(token).send(&mut guest.display.to_client);
    super::tell::configure(guest, id);
    repaint(guest);
}
