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

use nonos_toolkit::decorations::{content_rect, Rect};

use crate::app::App;
use crate::clients::compositor;
use crate::discover::Peers;
use crate::paint::PaintBuffer;

use super::boot::BootedApp;
use super::finish_band::finish_band;

/* Paint the app into its content area without touching the frame, then
 * restore the frame's rounded bottom corners over the rows the app redrew
 * and commit only the damaged rect, in screen coordinates. The app paints
 * even when the rect clips away, so what it planned to draw is drawn. */
pub(super) fn paint_partial<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: u32,
    r: Rect,
) {
    let b = &booted.binding;
    let c = content_rect(b.width, b.height, booted.maximized);
    let x = r.x.min(c.w);
    let y = r.y.min(c.h);
    let w = r.w.min(c.w - x);
    let h = r.h.min(c.h - y);
    let words = (b.byte_len / 4) as usize;
    let pixels: &mut [u32] =
        unsafe { core::slice::from_raw_parts_mut(b.backing_va as *mut u32, words) };
    let mut fb =
        PaintBuffer { pixels, stride_words: b.stride_words, width: b.width, height: b.height };
    booted.app.paint(&mut fb.sub(c.x, c.y, c.w, c.h));
    if w == 0 || h == 0 {
        return;
    }
    finish_band(&mut fb, booted.maximized, c.y + y, c.y + y + h);
    let (sx, sy) = (b.x + c.x + x, b.y + c.y + y);
    let _ = compositor::damage_commit(peers.compositor, request_id, sx, sy, w, h);
}
