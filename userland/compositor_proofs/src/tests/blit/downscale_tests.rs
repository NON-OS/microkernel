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

//! A canvas larger than the screen shrunk onto it: every screen pixel is the
//! average of the canvas pixels it covers, a one-pixel stroke survives as a
//! lighter one, and nothing outside the screen is written.

use crate::damage::Rect;
use crate::sw_blitter::{downscale, Surface};

const G: usize = 8;

fn surface(px: &mut [u32], w: u32, h: u32) -> Surface {
    let base = unsafe { px.as_mut_ptr().add(G) } as u64;
    Surface { base_va: base, stride: w * 4, width: w, height: h, byte_len: (w * h * 4) as u64 }
}

fn buf(w: u32, h: u32, fill: impl Fn(u32, u32) -> u32) -> Vec<u32> {
    let mut px = vec![0u32; G + (w * h) as usize + G];
    for y in 0..h {
        for x in 0..w {
            px[G + (y * w + x) as usize] = fill(x, y);
        }
    }
    px
}

fn guards(px: &[u32]) -> bool {
    px[..G].iter().chain(&px[px.len() - G..]).all(|&p| p == 0)
}

#[test]
fn a_flat_canvas_shrinks_to_the_same_colour_everywhere() {
    let (mut c, mut s) = (buf(1024, 768, |_, _| 0xFF20_4060), buf(800, 600, |_, _| 0));
    let full = Rect { x: 0, y: 0, width: 1024, height: 768 };
    let r = downscale(surface(&mut c, 1024, 768), surface(&mut s, 800, 600), full).unwrap();
    assert_eq!((r.x, r.y, r.width, r.height), (0, 0, 800, 600));
    assert!(s[G..s.len() - G].iter().all(|&p| p == 0xFF20_4060));
    assert!(guards(&s));
}

#[test]
fn a_one_pixel_stroke_survives_every_column() {
    for col in 0..1024u32 {
        let mut c = buf(1024, 4, |x, _| if x == col { 0xFFFF_FFFF } else { 0xFF00_0000 });
        let mut s = buf(800, 3, |_, _| 0);
        let full = Rect { x: 0, y: 0, width: 1024, height: 4 };
        downscale(surface(&mut c, 1024, 4), surface(&mut s, 800, 3), full).unwrap();
        assert!(s[G..G + 800].iter().any(|&p| p & 0xFF > 0), "column {col} vanished");
    }
}

#[test]
fn damage_writes_only_the_pixels_it_covers() {
    let (mut c, mut s) = (buf(1024, 768, |_, _| 0xFFFF_FFFF), buf(800, 600, |_, _| 0));
    let r = Rect { x: 512, y: 384, width: 10, height: 10 };
    let out = downscale(surface(&mut c, 1024, 768), surface(&mut s, 800, 600), r).unwrap();
    assert_eq!((out.x, out.y, out.width, out.height), (400, 300, 8, 8));
    assert_eq!(s[G..s.len() - G].iter().filter(|&&p| p != 0).count(), 64);
    let past = Rect { x: 1024, y: 0, width: 4, height: 4 };
    assert!(downscale(surface(&mut c, 1024, 768), surface(&mut s, 800, 600), past).is_none());
}
