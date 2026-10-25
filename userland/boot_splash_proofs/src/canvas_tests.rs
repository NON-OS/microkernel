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
//! The splash paints its whole canvas: drawn over a screen the firmware
//! left (a logo, a pattern), it is the drawing it makes on a blank surface,
//! so no firmware pixel shows; NONOS's own title is drawn; nothing past a
//! row's last pixel is written.

use crate::attest_lines::Chain;
use crate::canvas::{draw, Canvas};
use crate::layout_fit::layout;

const MODES: [(u32, u32); 4] = [(640, 480), (1024, 768), (1280, 800), (1920, 1080)];
/// The title's colour (title.rs).
const ACCENT: u32 = 0xFF00_D4AA;
/// The spare pixels past each row, before and after.
const SPARE: u32 = 0x0BAD_F00D;

/// A screen as firmware leaves it: a pattern, a white box in the middle.
fn firmware(w: u32, h: u32, extra: u32) -> Vec<u32> {
    let (spx, mut px) = (w + extra, vec![SPARE; ((w + extra) * h) as usize]);
    for y in 0..h {
        for x in 0..w {
            let logo = x.abs_diff(w / 2) < 130 && y.abs_diff(h / 2) < 40;
            let mark = 0xFF00_0000 | (x * 37 % 251) << 16 | (y * 53 % 241) << 8;
            px[(y * spx + x) as usize] = if logo { 0xFFFF_FFFF } else { mark | 0x80 };
        }
    }
    px
}

#[test]
fn nothing_the_firmware_left_shows() {
    let chain = Some(Chain { secure_boot: 0, kernel_sig_ok: 1, zk_verified: 2 });
    for (w, h) in MODES {
        for dma_unconfined in [false, true] {
            let c = Canvas { fit: layout(w, h), chain, dma_unconfined };
            let (mut over, mut fresh) = (firmware(w, h, 3), vec![SPARE; ((w + 3) * h) as usize]);
            draw(&mut over, (w + 3) as usize, w, h, &c);
            draw(&mut fresh, (w + 3) as usize, w, h, &c);
            for y in 0..h {
                let row = (y * (w + 3)) as usize;
                assert_eq!(
                    over[row..row + w as usize],
                    fresh[row..row + w as usize],
                    "{w}x{h} {y}"
                );
                assert!(over[row + w as usize..row + w as usize + 3].iter().all(|&p| p == SPARE));
            }
        }
    }
}

#[test]
fn the_mark_is_nonos_own() {
    for (w, h) in MODES {
        let c = Canvas { fit: layout(w, h), chain: None, dma_unconfined: false };
        let mut px = vec![SPARE; (w * h) as usize];
        draw(&mut px, w as usize, w, h, &c);
        let title = (h / 2 - 132) as usize * w as usize..(h / 2 - 90) as usize * w as usize;
        assert!(px[title].iter().filter(|&&p| p == ACCENT).count() > 200, "{w}x{h}");
    }
}
