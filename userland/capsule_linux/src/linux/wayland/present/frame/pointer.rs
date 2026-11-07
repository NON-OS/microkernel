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

//! The pointer on a framed window: on the content it is the guest's, at the
//! content's own coordinates; on the frame it lights a control, presses a
//! window button, drags the window by its title bar, or resizes it by an
//! edge of its border. On a window that draws its own frame the band just
//! inside its edge resizes it, and the rest is the guest's.

use nonos_toolkit::decorations::{hit_test_at, DecorationHit, Edges, ARROW_CURSOR};

use crate::linux::guest::Guest;

use super::super::super::fit::Sides;
use super::super::super::input::cursor;
use super::super::super::scene::Scene;
use super::super::super::toplevel::resize;
use super::now::shown;
use super::redraw::light;
use super::Around;

/// The driver's left button.
const LEFT: u32 = 1;

/// A motion at (`x`, `y`) on the window: where on the content to tell the
/// guest, or None when the frame took it.
pub fn motion(guest: &mut Guest, x: i32, y: i32) -> Option<(i32, i32)> {
    let Some((a, (cw, ch, _))) = shown(&guest.scene) else { return Some((x, y)) };
    guest.scene.win.frame.pointer = (x, y);
    if let Some(drag) = guest.scene.win.frame.drag {
        super::drag::to(&mut guest.scene, drag, (x, y), a);
        return None;
    }
    // A held edge takes every motion until it is let go.
    if guest.scene.win.frame.resize.is_some() {
        return None;
    }
    // On the content, but not where the frame's edges reach in over it
    // (Around::edges), unless a button the client holds keeps the pointer,
    // or the window is not one the frame sizes.
    let held = guest.scene.dnd.pressed;
    let q = guest.scene.win.frame.quarters;
    let sizes = resize::shape(&guest.scene, a, (x, y)) != ARROW_CURSOR;
    let over = edges(&guest.scene, a, (x, y), (cw, ch)).filter(|_| !held && sizes);
    if let Some(on) = a.on_content(x, y, cw, ch).filter(|_| over.is_none()) {
        light(&mut guest.scene, DecorationHit::None);
        if core::mem::take(&mut guest.scene.win.frame.shaped) {
            cursor::frame(guest, ARROW_CURSOR);
        }
        return Some(on);
    }
    // Off the client's surface first, which puts the arrow back, then the
    // shape the frame's edge there takes.
    super::super::super::input::off::pointer_off(guest);
    let shape = over.map_or_else(|| resize::shape(&guest.scene, a, (x, y)), |e| e.cursor());
    guest.scene.win.frame.shaped = shape != ARROW_CURSOR;
    cursor::frame(guest, shape);
    // A window that draws its own frame has no control of NØNOS's to light.
    let hit = match x >= 0 && y >= 0 && a.framed() {
        true => hit_test_at(a.w, a.h, a.max, x as u32, y as u32, q),
        false => DecorationHit::None,
    };
    light(&mut guest.scene, hit);
    None
}

/// A button press or release: true when the frame took it.
pub fn button(guest: &mut Guest, code: u32, down: bool) -> bool {
    let Some((a, (cw, ch, _))) = shown(&guest.scene) else { return false };
    let (x, y) = guest.scene.win.frame.pointer;
    if !down && guest.scene.win.frame.drag.take().is_some() {
        return true;
    }
    if guest.scene.win.frame.resize.is_some() {
        // Held until its button is let go; another press meanwhile is the
        // frame's too, and ends nothing.
        if let Some(r) = guest.scene.win.frame.resize.filter(|_| !down) {
            guest.scene.win.frame.resize = None;
            resize::end(guest, r);
        }
        return true;
    }
    // The edges reach in over the content's outer pixels, and over a window
    // that draws its own frame its band just inside: a left press there
    // resizes; anything else on the content, its release included, is the
    // client's.
    if a.on_content(x, y, cw, ch).is_some() {
        return down && code == LEFT && begin_edges(guest, a, (cw, ch), (x, y));
    }
    if !a.framed() {
        return false;
    }
    if !down || code != LEFT || x < 0 || y < 0 {
        return true;
    }
    if begin_edges(guest, a, (cw, ch), (x, y)) {
        return true;
    }
    let (px, py, q) = (x as u32, y as u32, guest.scene.win.frame.quarters);
    match hit_test_at(a.w, a.h, a.max, px, py, q) {
        DecorationHit::CloseButton => super::buttons::close(guest),
        DecorationHit::MinimizeButton => super::buttons::minimize(&mut guest.scene),
        DecorationHit::MaximizeButton => super::buttons::maximize(guest),
        DecorationHit::Titlebar => {
            // The window moves: its popups end now, as at a minimise, not a
            // frame or more later at the window manager's notice, drawn at
            // their old place meanwhile.
            super::super::super::popup::ends::end_all(guest);
            super::drag::begin(&mut guest.scene, (x, y));
        }
        DecorationHit::None => {}
    }
    true
}

/// A resize from the edges a press at (`x`, `y`) grabs; false when it grabs
/// none or the window is not one the frame sizes (toplevel/resize.rs).
fn begin_edges(guest: &mut Guest, a: Around, content: (u32, u32), (x, y): (i32, i32)) -> bool {
    let Some(e) = edges(&guest.scene, a, (x, y), content) else { return false };
    let sides = Sides { left: e.left, right: e.right, top: e.top, bottom: e.bottom };
    resize::begin(guest, sides, (x, y))
}

/// The edges a press at (`x`, `y`) grabs: the frame's on a framed window,
/// the band just inside its edge on one that draws its own frame, and none
/// on any other bare window (full screen, or one the client frames for no
/// one to resize).
fn edges(scene: &Scene, a: Around, (x, y): (i32, i32), content: (u32, u32)) -> Option<Edges> {
    let q = scene.win.frame.quarters;
    match a.framed() {
        true => a.edges(x, y, content, q),
        false if scene.win.frame.own_frame => a.inner_edges(x, y, q),
        false => None,
    }
}
