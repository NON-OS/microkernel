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
//! A resize of the shown window, begun by a press on the frame's border, by
//! one just inside the edge of a window that draws its own frame, or by
//! xdg_toplevel.resize from a client that draws its own, held until the
//! button is let go, and then told to the guest as its new size
//! (fit/resize.rs). Resizing was the window manager's alone, which a Linux
//! window never reached: dragging its edge did nothing.

use nonos_app_skeleton::runner::chrome::menubar_for;
use nonos_toolkit::decorations::{edges_at, ARROW_CURSOR};

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::configure::configure;
use super::super::fit::{sized, Held, Mode, Resize, Sides};
use super::super::present::frame::now::{outer_rect, shown};
use super::super::present::frame::Around;
use super::super::present::scale_math::point_px;
use super::super::scene::Scene;
use super::ask::logical;

/// Hold `sides` of the shown window, pressed at `at` on it. False when it
/// is not in its own size, or on its way out of it.
pub fn begin(guest: &mut Guest, sides: Sides, at: (i32, i32)) -> bool {
    let scene = &mut guest.scene;
    if !sides.any() || !own_size(scene) {
        return false;
    }
    let Some(base) = outer_rect(scene) else { return false };
    let (w, h) = super::super::output::screen();
    scene.win.frame.top = menubar_for(w, h);
    let limits = limits(scene);
    scene.win.frame.resize = Some(Resize { sides, press: at, base, limits });
    // Its popups end as the resize begins, as they do at a move (moving.rs).
    super::super::popup::ends::end_all(guest);
    true
}

/// Whether the shown window is in its own size and not on its way out of
/// it: the only one whose edges move.
fn own_size(scene: &Scene) -> bool {
    let fit = &scene.win.fit;
    fit.now() == Mode::Normal && fit.heading() == Mode::Normal
}

/// The pointer's shape at (`x`, `y`) on the frame `a`: the resize a press
/// there makes, or the arrow where it makes none.
pub fn shape(scene: &Scene, a: Around, (x, y): (i32, i32)) -> u8 {
    if !own_size(scene) || x < 0 || y < 0 {
        return ARROW_CURSOR;
    }
    let q = scene.win.frame.quarters;
    match a.framed() {
        true => edges_at(a.w, a.h, a.max, x as u32, y as u32, q).cursor(),
        // A window that draws its own frame: the band just inside its edge.
        false if scene.win.frame.own_frame => {
            a.inner_edges(x, y, q).map_or(ARROW_CURSOR, |e| e.cursor())
        }
        false => ARROW_CURSOR,
    }
}

/// The least and most content the shown window's client set, in the
/// logical pixels it speaks, as shown pixels at the window's ratio.
fn limits(scene: &Scene) -> ((u32, u32), (u32, u32)) {
    let surface = scene.win.shown.map(|w| w.surface);
    let Some(s) = scene.surfaces.iter().find(|s| Some(s.id) == surface) else {
        return ((0, 0), (0, 0));
    };
    let e = scene.win.frame.ratio;
    let px = |(w, h): (u32, u32)| {
        let clamp = |v: u32| v.min(i32::MAX as u32) as i32;
        let (w, h) = point_px((clamp(w), clamp(h)), e);
        (w.max(0) as u32, h.max(0) as u32)
    };
    (px(s.limits.0), px(s.limits.1))
}

/// The resize `r` let go where the pointer last was: the guest told its new
/// size, and the corner its window goes to with the buffer drawn for it.
pub fn end(guest: &mut Guest, r: Resize) {
    let Some((a, (cw, ch, _))) = shown(&guest.scene) else { return };
    let Some(on) = guest.scene.win.shown else { return };
    let Some(toplevel) = on.toplevel else { return };
    let xdg = guest.scene.surfaces.iter().find(|s| s.id == on.surface).and_then(|s| s.xdg);
    let Some(xdg) = xdg else { return };
    let frame = (a.w.saturating_sub(cw), a.h.saturating_sub(ch));
    let (pointer, top) = (guest.scene.win.frame.pointer, guest.scene.win.frame.top);
    let rect = sized(&r, pointer, frame, top, super::super::output::screen());
    if rect == (r.base.0, r.base.1, cw, ch) {
        return;
    }
    let serial = guest.scene.next_serial();
    let held = Held::of(&r, rect, frame, top);
    let Some(told) = guest.scene.win.fit.resize(rect, held, serial) else { return };
    guest.scene.win.fit_of = Some(toplevel);
    let told = logical(told, guest.scene.win.frame.ratio);
    let how = (guest.scene.win.focused, super::bounds::of(guest, toplevel));
    configure(toplevel, xdg, &told, how, &mut guest.display.to_client);
}

/// xdg_toplevel.resize: taken, as a move is (moving.rs), with the serial of
/// the latest press while the button is held, on the shown window; its edge
/// is then held as the frame's border holds one, and the pointer leaves the
/// client's surface while it is.
pub fn resizing(guest: &mut Guest, toplevel: u32, args: &mut Args<'_>) {
    let (_seat, serial, edge) = (args.u32(), args.u32(), args.u32());
    let scene = &guest.scene;
    let held = scene.dnd.pressed && serial.is_some() && scene.popups.last_press == serial;
    let ours = scene.win.shown.and_then(|w| w.toplevel) == Some(toplevel);
    let (Some(edge), true) = (edge, held && ours) else { return };
    let at = scene.win.frame.pointer;
    if begin(guest, Sides::of_edge(edge), at) {
        super::super::input::off::pointer_off(guest);
    }
}
