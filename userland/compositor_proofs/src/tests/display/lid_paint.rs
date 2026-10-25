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

//! A lid rig frame's pixels: a rectangle composed as the compositor composes
//! it under the lid, and a batch presented onto the rig's screen.

use super::lid_rig::Rig;
use crate::damage::Rect;
use crate::frame_pacer::indicator_band::draw;
use crate::state::indicator::Indicators;
use crate::state::lid_rule::{Lid, LID_BLACK};
use crate::sw_blitter::{fill_rect, Surface};

pub(super) const W: u32 = 160;
pub(super) const H: u32 = 48;
const DESK: u32 = 0xFF20_4060;

#[derive(Clone, Copy, Default)]
pub(super) struct Shows {
    pub(super) ind: Indicators,
    pub(super) captured: bool,
    pub(super) trusted: bool,
}

/// The whole screen black, as a dark lid leaves it.
pub(super) fn black() -> Vec<u32> {
    vec![LID_BLACK; (W * H) as usize]
}

pub(super) fn paint(px: &mut [u32], clip: Rect, lid: Lid, s: Shows) {
    let byte_len = px.len() as u64 * 4;
    let dst =
        Surface { base_va: px.as_mut_ptr() as u64, stride: W * 4, width: W, height: H, byte_len };
    if lid.black() {
        return fill_rect(dst, clip, LID_BLACK);
    }
    fill_rect(dst, clip, DESK);
    draw(dst, clip, &s.ind, s.captured, s.trusted, false);
}

/// The screen drawn whole for `lid` and what the band shows.
pub(super) fn whole(lid: Lid, s: Shows) -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    paint(&mut px, Rect { x: 0, y: 0, width: W, height: H }, lid, s);
    px
}

pub(super) fn present(r: &mut Rig, rects: &[Rect]) -> Result<(), ()> {
    if r.refuse > 0 {
        r.refuse -= 1;
        return Err(());
    }
    r.presents += 1;
    for rect in rects {
        for y in rect.y..rect.y + rect.height {
            let row = (y * W) as usize;
            let (a, b) = (row + rect.x as usize, row + (rect.x + rect.width) as usize);
            r.screen[a..b].copy_from_slice(&r.canvas[a..b]);
        }
    }
    Ok(())
}
