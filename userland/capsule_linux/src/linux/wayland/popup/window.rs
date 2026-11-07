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
//! A popup in a window of its own over its parent's (OP_POPUP_OPEN): the
//! window manager places it against the parent's window by its positioner,
//! on the whole output, so a menu longer than its window runs past it; it
//! never takes focus, and it closes with its parent or, with a grab, at a
//! press outside its chain (POPUP_DONE, input/focus.rs). Its pixels are on
//! a surface and layer of its own (present.rs). Without a window manager
//! that places popups, a popup is laid on its window instead (layer.rs).

use nonos_app_skeleton::clients::compositor::scene_remove_layer;
use nonos_app_skeleton::clients::wm::{popup_open, window_close, PopupSpec, POPUP_GRAB};
use nonos_app_skeleton::discover::lookup_port;

use crate::linux::guest::Guest;

use super::super::present::frame::now::shown;
use super::super::present::scale_math::{point_px, to_logical};
use super::super::present::window::APP_LAYER_Z;
use super::super::reshape::Shape;
use super::super::scene::Scene;
use super::super::scene_pixels::Pixels;
use super::super::window_front::ids::{free_id, GUEST_POPUPS};
use super::made::origin;

pub struct PopupWindow {
    /// The xdg_popup, and its surface.
    pub popup: u32,
    pub surface: u32,
    /// The window manager's id for it.
    pub window: u32,
    /// Where the window manager placed it on screen.
    pub at: (u32, u32),
    /// The NONOS surface its pixels are on, its shape, and the pixels.
    pub out: Option<u64>,
    pub shape: Option<Shape>,
    pub pixels: Pixels,
}

/// Popup `id` opened in a window of its own, `grab` or not, its rect set to
/// where the window manager placed it; false when none placed it.
pub fn open(guest: &mut Guest, id: u32, grab: bool) -> bool {
    let Some(p) = guest.scene.popups.list.iter().find(|p| p.id == id) else { return false };
    let (surface, parent, rule) = (p.surface, p.parent, p.rule);
    let Some((parent_id, (cx, cy))) = parent_window(&guest.scene, parent) else { return false };
    let Some(wm) = lookup_port(b"wm") else { return false };
    let e = guest.scene.win.frame.ratio;
    let px = |x: i32, y: i32| point_px((x, y), e);
    let ((ax, ay), (aw, ah)) = (px(rule.rect.0, rule.rect.1), px(rule.rect.2, rule.rect.3));
    let ((w, h), (ox, oy)) = (px(rule.size.0, rule.size.1), px(rule.offset.0, rule.offset.1));
    // One no other popup window of this guest has (window_front/ids.rs).
    let used: alloc::vec::Vec<u32> = guest.scene.popup_windows.iter().map(|w| w.window).collect();
    let Some(window) = free_id(GUEST_POPUPS, &used) else { return false };
    let spec = PopupSpec {
        window_id: window,
        parent_id,
        // The client sets the rect, up to i32::MAX: held there, not wrapped.
        anchor_x: cx.saturating_add(ax),
        anchor_y: cy.saturating_add(ay),
        anchor_w: aw.max(0) as u32,
        anchor_h: ah.max(0) as u32,
        width: w.max(1) as u32,
        height: h.max(1) as u32,
        anchor: rule.anchor,
        gravity: rule.gravity,
        adjust: rule.adjust,
        offset_x: ox,
        offset_y: oy,
        flags: if grab { POPUP_GRAB } else { 0 },
    };
    let rid = guest.scene.next_serial();
    let Ok(placed) = popup_open(wm, rid, &spec) else { return false };
    // Told on the toplevel's surface, in the logical pixels it draws in.
    let (px0, py0) = origin(&guest.scene, parent).unwrap_or((0, 0));
    let rel = (to_logical(placed.rel_x - cx, e), to_logical(placed.rel_y - cy, e));
    let size = (to_logical(placed.width as i32, e), to_logical(placed.height as i32, e));
    if let Some(p) = guest.scene.popups.list.iter_mut().find(|p| p.id == id) {
        p.rect = (px0.saturating_add(rel.0), py0.saturating_add(rel.1), size.0, size.1);
    }
    guest.scene.popup_windows.push(PopupWindow {
        popup: id,
        surface,
        window,
        at: (placed.x, placed.y),
        out: None,
        shape: None,
        pixels: Pixels::empty(),
    });
    true
}

/// The window manager window popup `id`'s parent surface `parent` is on,
/// and where that surface's content starts in it: a popup's own, or the
/// toplevel's inside its frame.
fn parent_window(scene: &Scene, parent: u32) -> Option<(u32, (i32, i32))> {
    if let Some(w) = scene.popup_windows.iter().find(|w| w.surface == parent) {
        return Some((w.window, (0, 0)));
    }
    if scene.win.shown.map(|s| s.surface) != Some(parent) {
        return None;
    }
    let (a, _) = shown(scene)?;
    Some((scene.win.window?, (a.cx as i32, a.cy as i32)))
}

/// Popup `id`'s window closed, its layer gone and its surface released.
pub fn close(scene: &mut Scene, id: u32) {
    let Some(at) = scene.popup_windows.iter().position(|w| w.popup == id) else { return };
    let w = scene.popup_windows.remove(at);
    if let (Some(port), Some(handle)) = (lookup_port(b"compositor"), w.out) {
        let _ = scene_remove_layer(port, scene.next_serial(), handle, APP_LAYER_Z);
    }
    if let Some(wm) = lookup_port(b"wm") {
        let _ = window_close(wm, scene.next_serial(), w.window);
    }
    if let Some(handle) = w.out {
        if nonos_libc::mk_surface_release(handle) < 0 {
            core::mem::forget(w.pixels);
        }
    }
}

/// The popup window manager window `id` is: its popup's surface and rect.
pub fn of_window(scene: &Scene, id: u32) -> Option<(u32, (i32, i32, i32, i32))> {
    let w = scene.popup_windows.iter().find(|w| w.window == id)?;
    let p = scene.popups.list.iter().find(|p| p.id == w.popup)?;
    Some((p.surface, p.rect))
}
