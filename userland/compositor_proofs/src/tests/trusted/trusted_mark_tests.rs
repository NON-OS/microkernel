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

//! The trusted mark: the compositor's own, in the band no surface can cover.
//! It is white only while the trusted band is held, and a full-screen layer
//! in the trusted band, the lock screen, still leaves the band on top.

use crate::damage::Rect;
use crate::frame_pacer::indicator_band::{band, draw, MARK_OFF, TRUSTED_ON};
use crate::state::indicator::Indicators;
use crate::sw_blitter::{fill_rect, Surface};

const W: u32 = 160;
const H: u32 = 40;
const LOCK: u32 = 0xFF07_0B10;

fn screen(px: &mut [u32]) -> Surface {
    Surface {
        base_va: px.as_mut_ptr() as u64,
        stride: W * 4,
        width: W,
        height: H,
        byte_len: px.len() as u64 * 4,
    }
}

fn mark(px: &[u32]) -> u32 {
    let b = band(W, H);
    px[(5 * W + b.x + 69) as usize]
}

#[test]
fn the_mark_is_lit_only_while_the_band_is_held() {
    for (held, want) in [(false, MARK_OFF), (true, TRUSTED_ON)] {
        let mut px = vec![0u32; (W * H) as usize];
        draw(
            screen(&mut px),
            Rect { x: 0, y: 0, width: W, height: H },
            &Indicators::default(),
            false,
            held,
            false,
        );
        assert_eq!(mark(&px), want, "held {held}");
    }
}

#[test]
fn the_lock_screen_leaves_the_band_on_top() {
    let mut px = vec![0u32; (W * H) as usize];
    let whole = Rect { x: 0, y: 0, width: W, height: H };
    // The lock fills the screen; the band is drawn after it, as every frame does.
    fill_rect(screen(&mut px), whole, LOCK);
    draw(screen(&mut px), whole, &Indicators::default(), false, true, false);
    assert_eq!(mark(&px), TRUSTED_ON);
    let b = band(W, H);
    assert!((b.y..b.y + b.height).all(|y| (b.x..W).all(|x| px[(y * W + x) as usize] != LOCK)));
    assert_eq!(px[(H / 2 * W) as usize], LOCK, "the rest of the screen is the lock's");
}
