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

//! xdg_surface.get_popup. A popup is placed when it is made, against its
//! parent, the window's toplevel or a popup open on it, and within the
//! window. One that cannot be, its surface with a role already, its parent
//! or a sized positioner unknown, or no window shown, ends at once.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::object::Object;
use super::super::ops::shell::POPUP_DONE;
use super::super::out::Event;
use super::super::present::scale_math::to_logical;
use super::super::scene::Scene;
use super::place::place;
use super::positioner::Positioner;
use super::side::complete;
use super::stack::Popup;

/// get_popup on xdg_surface `xdg`: the new popup, its parent's xdg_surface
/// and its positioner.
pub fn get_popup(guest: &mut Guest, xdg: u32, args: &mut Args<'_>) {
    let (Some(id), Some(parent), Some(positioner)) = (args.u32(), args.u32(), args.u32()) else {
        return;
    };
    if !guest.objects.put(id, Object::Popup) {
        return;
    }
    // Placed against its parent's window, whichever window committed last.
    super::super::window_front::select_role(guest, parent);
    match placed(&guest.scene, id, xdg, parent, positioner) {
        Some(p) => guest.scene.popups.list.push(p),
        None => Event::new(id, POPUP_DONE).send(&mut guest.display.to_client),
    }
}

fn placed(scene: &Scene, id: u32, xdg: u32, parent: u32, positioner: u32) -> Option<Popup> {
    let surface_of = |x: u32| scene.surfaces.iter().find(|s| s.xdg == Some(x)).map(|s| s.id);
    let (surface, parent) = (surface_of(xdg)?, surface_of(parent)?);
    let rule = scene.popups.positioners.iter().find(|(p, _)| *p == positioner)?.1;
    let toplevel = scene.surfaces.iter().any(|s| s.id == surface && s.toplevel.is_some());
    let role = toplevel || scene.subs.of(surface).is_some() || origin(scene, surface).is_some();
    if role {
        return None;
    }
    let rect = rect(scene, parent, &rule)?;
    Some(Popup { id, surface, parent, rect, grabbed: false, shown: false, rule })
}

/// Where `rule` places a popup on surface `parent`, on the toplevel's
/// surface and within the window; None for a positioner xdg-shell calls
/// incomplete, or a parent not on the shown window.
pub fn rect(scene: &Scene, parent: u32, rule: &Positioner) -> Option<(i32, i32, i32, i32)> {
    let (ox, oy) = origin(scene, parent)?;
    if !complete(rule) {
        return None;
    }
    let (_, _, w, h) = scene.rect()?;
    // Placed in the logical pixels the client draws in.
    let e = scene.win.frame.ratio;
    let (w, h) = (to_logical(w as i32, e), to_logical(h as i32, e));
    // The window, from the parent's corner: it starts at the toplevel's
    // window geometry (present/crop.rs).
    let (wx, wy) = scene.win.origin;
    let (x, y, pw, ph) = place(rule, (wx.saturating_sub(ox), wy.saturating_sub(oy), w, h));
    Some((x.saturating_add(ox), y.saturating_add(oy), pw, ph))
}

/// Where surface `parent`'s corner a popup is placed from sits on the
/// window's toplevel surface: the toplevel's window geometry, or a popup
/// open on it.
pub fn origin(scene: &Scene, parent: u32) -> Option<(i32, i32)> {
    match scene.popups.list.iter().find(|p| p.surface == parent) {
        Some(p) => Some((p.rect.0, p.rect.1)),
        None => (Some(parent) == scene.win.shown.map(|s| s.surface)).then_some(scene.win.origin),
    }
}
