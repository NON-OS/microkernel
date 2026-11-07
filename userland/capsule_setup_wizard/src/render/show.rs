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

//! A setup frame, drawn off screen, put on the surface the compositor reads
//! row by row where it differs, and committed as those rows' bands
//! (row_bands.rs). Drawn on the surface itself, when no off-screen frame
//! could be mapped at start, it is committed whole, as before.

use super::row_bands::Bands;
use crate::clients::compositor;
use crate::state::Context;

/// Show the frame the screen's draw left off screen and commit what changed.
pub fn show(ctx: &Context, request_id: u32) {
    for b in copy_changed(ctx).as_slice() {
        let (y, rows) = (b.y as u32, b.rows as u32);
        let _ = compositor::damage_rect(ctx.compositor_port, request_id, 0, y, ctx.width, rows);
    }
}

fn copy_changed(ctx: &Context) -> Bands {
    let rows = ctx.height as usize;
    if ctx.back == 0 {
        return Bands::whole(rows);
    }
    let spx = ctx.stride as usize / 4;
    let w = (ctx.width as usize).min(spx);
    let Some(words) = spx.checked_mul(rows) else {
        return Bands::whole(rows);
    };
    // SAFETY: `base` and `back` are two separate private mappings setup made
    // at start, each `stride * height` bytes (setup/run.rs), so each holds
    // `words` pixels and neither slice aliases the other. Only this process
    // writes either; the compositor reads `base` through its own read-only
    // view, and nothing here keeps another reference to either while these
    // live.
    let (shown, back) = unsafe {
        (
            core::slice::from_raw_parts_mut(ctx.base as *mut u32, words),
            core::slice::from_raw_parts(ctx.back as *const u32, words),
        )
    };
    let mut bands = Bands::new();
    for (y, (s, b)) in shown.chunks_mut(spx).zip(back.chunks(spx)).enumerate() {
        let (s, b) = (&mut s[..w], &b[..w]);
        if s != b {
            s.copy_from_slice(b);
            bands.add(y);
        }
    }
    bands
}
