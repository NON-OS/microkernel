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

//! The blitter against its routines as they stood before the change
//! (bench/reference.rs): the same pixels for every rectangle and scale.

use crate::bench::reference::{self, surface};
use crate::damage::Rect;
use crate::sw_blitter::{scaled, upscale};
use crate::tests::scene::frame_util::Rng;

fn word(r: &mut Rng) -> u32 {
    r.below(u32::MAX) ^ r.below(u32::MAX) << 16
}

const CW: u32 = 61;
const CH: u32 = 37;

fn canvas(r: &mut Rng) -> Vec<u32> {
    (0..CW * CH).map(|_| word(r)).collect()
}

fn rects(r: &mut Rng) -> Vec<Rect> {
    let mut v = vec![
        Rect { x: 0, y: 0, width: CW, height: CH },
        Rect { x: CW - 1, y: CH - 1, width: 9, height: 9 },
    ];
    for _ in 0..60 {
        let (x, y) = (word(r) % CW, word(r) % CH);
        v.push(Rect { x, y, width: word(r) % 40 + 1, height: word(r) % 30 + 1 });
    }
    v
}

#[test]
fn an_upscale_shows_what_the_old_one_showed() {
    let mut r = Rng::new(41);
    let mut src = canvas(&mut r);
    for scale in [1, 2, 3, 4] {
        // A screen a little short of the scaled canvas, so edges clip.
        let (sw, sh) = (CW * scale - 3, CH * scale - 2);
        for rect in rects(&mut r) {
            let (mut a, mut b) = (vec![7u32; (sw * sh) as usize], vec![7u32; (sw * sh) as usize]);
            let c = surface(&mut src, CW, CH);
            let got = upscale(c, surface(&mut a, sw, sh), rect, scale);
            let want = reference::upscale(c, surface(&mut b, sw, sh), rect, scale);
            assert_eq!(
                got.map(|r| (r.x, r.y, r.width, r.height)),
                want.map(|r| (r.x, r.y, r.width, r.height))
            );
            assert!(a == b, "scale {scale}, rect {:?}", (rect.x, rect.y, rect.width, rect.height));
        }
    }
}

#[test]
fn a_stretch_shows_what_the_old_one_showed() {
    let mut r = Rng::new(43);
    let mut src = canvas(&mut r);
    for q in [5, 6, 7, 9, 10, 11, 13] {
        let (sw, sh) = (CW * q / 4 + 1, CH * q / 4 - 1);
        for rect in rects(&mut r) {
            let (mut a, mut b) = (vec![7u32; (sw * sh) as usize], vec![7u32; (sw * sh) as usize]);
            let c = surface(&mut src, CW, CH);
            let got = scaled(c, surface(&mut a, sw, sh), rect, q);
            let want = reference::stretch(c, surface(&mut b, sw, sh), rect, q);
            assert_eq!(
                got.map(|r| (r.x, r.y, r.width, r.height)),
                want.map(|r| (r.x, r.y, r.width, r.height))
            );
            assert!(a == b, "q {q}, rect {:?}", (rect.x, rect.y, rect.width, rect.height));
        }
    }
}

#[test]
fn a_layer_composites_as_the_old_row_did() {
    let mut r = Rng::new(47);
    for _ in 0..400 {
        let w = (word(&mut r) as usize) % 300 + 1;
        let src: Vec<u32> = (0..w)
            .map(|i| {
                let px = word(&mut r);
                match (i / 7 + (word(&mut r) as usize)) % 3 {
                    0 => px | 0xFF00_0000,
                    1 => px & 0x00FF_FFFF,
                    _ => px,
                }
            })
            .collect();
        let dst: Vec<u32> = (0..w).map(|_| word(&mut r)).collect();
        let (mut a, mut b) = (dst.clone(), dst);
        crate::sw_blitter::row::composite_row(&src, &mut a);
        reference::composite_row(&src, &mut b);
        assert_eq!(a, b);
    }
}

#[test]
fn a_stretch_with_no_room_for_its_table_shows_the_same() {
    let no_room: crate::sw_blitter::stretch::Table = |_| None;
    let mut r = Rng::new(53);
    let mut src = canvas(&mut r);
    for q in [5, 6, 7, 10] {
        let (sw, sh) = (CW * q / 4, CH * q / 4);
        for rect in rects(&mut r) {
            let (mut a, mut b) = (vec![7u32; (sw * sh) as usize], vec![7u32; (sw * sh) as usize]);
            let c = surface(&mut src, CW, CH);
            let got = crate::sw_blitter::stretch::stretch_with(
                c,
                surface(&mut a, sw, sh),
                rect,
                q,
                no_room,
            );
            let want = reference::stretch(c, surface(&mut b, sw, sh), rect, q);
            assert_eq!(
                got.map(|r| (r.x, r.y, r.width, r.height)),
                want.map(|r| (r.x, r.y, r.width, r.height))
            );
            assert!(a == b, "q {q}, rect {:?}", (rect.x, rect.y, rect.width, rect.height));
        }
    }
}
