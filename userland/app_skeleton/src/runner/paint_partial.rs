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

use nonos_toolkit::decorations::{content_rect_at, FrameState, Rect};

use crate::app::App;
use crate::discover::Peers;
use crate::paint::PaintBuffer;
use crate::swap::Damage;

use super::boot::BootedApp;
use super::finish_band::finish_band;
use super::paint_frame::draw_whole;
use super::present_frame::{present, ACQUIRE_MS};
use super::request_id::next;
use super::swap_paint::whole_needed;

/* Paint the app's damage into a buffer the compositor released, restore the
 * frame's rounded bottom corners where the app drew, and present only the
 * damaged rect, in the buffer's pixels. An app that names a rect paints that
 * rect alone, so when more of the buffer than the rect is out of date the
 * app is told to draw everything and the whole window is drawn instead
 * (`swap_paint`); it is still the rect alone that changed on screen. The
 * app paints even when the rect clips away, so what it planned to draw is
 * drawn; nothing is presented then. False when no buffer came free or the
 * present was refused. */
pub(super) fn paint_partial<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
    r: Rect,
    frame: FrameState,
) -> bool {
    let maximized = booted.maximized;
    let b = &mut booted.binding;
    let c = content_rect_at(b.width, b.height, maximized, super::chrome::quarters());
    let x = r.x.min(c.w);
    let y = r.y.min(c.h);
    let w = r.w.min(c.w - x);
    let h = r.h.min(c.h - y);
    let d = Damage { x: c.x + x, y: c.y + y, width: w, height: h };
    let Some((i, stale)) = b.swap.acquire(ACQUIRE_MS) else { return false };
    if whole_needed(stale, (d.x, d.y, d.width, d.height)) {
        booted.app.paint_whole_next();
        draw_whole(&mut booted.app, &booted.manifest, b, i, (frame, maximized));
    } else {
        let (stride_words, width, height) = (b.stride_words, b.width, b.height);
        let mut fb = PaintBuffer { pixels: b.pixels(i), stride_words, width, height };
        booted.app.paint(&mut fb.sub(c.x, c.y, c.w, c.h));
        let drawn = match booted.app.painted_damage_alone() {
            true => (d.x, d.y, d.width, d.height),
            false => (c.x, c.y, c.w, c.h),
        };
        finish_band(&mut fb, maximized, corner_box(drawn));
    }
    if w == 0 || h == 0 {
        return b.swap.cancel(i);
    }
    present(b, peers.compositor, next(request_id), i, d)
}

/// The box [x0, x1) x [y0, y1) a paint of (x, y, width, height) drew.
fn corner_box((x, y, w, h): (u32, u32, u32, u32)) -> (u32, u32, u32, u32) {
    (x, y, x.saturating_add(w), y.saturating_add(h))
}
