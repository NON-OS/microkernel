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

//! Getting the client's pixels onto a NONOS surface.

use crate::linux::guest::Guest;

use super::super::reshape::{reshape, Reshape};
use super::super::scene::Scene;
use super::super::swap_copy::Area;
use super::draw::{draw, whole, Drew};
use super::frame::fill::{fill, window_bytes};
use super::frame::geometry::Around;
use super::no_room::outgrown;
use super::refit::refit;
use super::source::Source;
use super::window::show;

/// Show `buffer`, committed on `surface`, whole.
pub fn present(guest: &mut Guest, surface: u32, buffer: u32) -> Drew {
    let src = match super::scaled::window(&mut guest.scene, &guest.memfd_spans, surface, buffer) {
        Ok(found) => found,
        Err(why) => {
            say(why);
            return Drew::Nothing;
        }
    };
    // Presented by the compositor, as every other app's window is; the
    // personality holds no GfxPresent and needs none. A buffer of a shape
    // the surface does not have gets one that has it (reshape.rs).
    if reshape(guest.scene.win.shape, src.shape) != Reshape::Same {
        super::super::popup::ends::end_all(guest);
        return show(guest, surface, &src);
    }
    // Into the window's frame, painted around the guest's pixels (frame/).
    let a = super::frame::now::around_shape(&guest.scene, src.shape);
    let bytes = window_bytes(a, &src);
    if !guest.scene.win.bufs.fits(bytes) {
        // The frame outgrew its buffers (a maximise, a new scale): the
        // window gets buffers of its new size.
        outgrown();
        return show(guest, surface, &src);
    }
    // The frame took another shape with the content's size kept (a
    // maximise or full screen acked, the client's buffer the same): the
    // bytes may fit, but rows drawn at the new stride into buffers laid at
    // the old one would shear, so the window gets buffers of its new shape.
    if guest.scene.win.bufs.shape() != a.laid(src.shape) {
        return show(guest, surface, &src);
    }
    let pid = guest.pid;
    guest.scene.win.origin = src.origin;
    let cover = covered(&guest.scene, a, &src);
    let all = whole((a.w, a.h));
    let drew = draw(&mut guest.scene, bytes, cover, |scene, out| {
        fill(scene, (pid, surface), &src, out, a).then_some(all)
    });
    if drew != Drew::Shown {
        return drew;
    }
    guest.scene.win.owing = None;
    if guest.scene.win.fit.due().is_some() {
        refit(&mut guest.scene);
    }
    drew
}

/// What a fill writes every pixel of: the whole window, but the frame when
/// it is not painted again, which is only the content then.
fn covered(scene: &Scene, a: Around, src: &Source) -> Area {
    let key = (a, scene.win.frame.hover, scene.win.focused);
    if !a.framed() || scene.win.frame.chrome != Some(key) {
        return whole((a.w, a.h));
    }
    let (cw, ch) =
        (src.shape.0.min(a.w.saturating_sub(a.cx)), src.shape.1.min(a.h.saturating_sub(a.cy)));
    (a.cx, a.cy, cw, ch)
}

fn say(line: &[u8]) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
