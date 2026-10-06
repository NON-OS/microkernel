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

//! Chrome and UI text (ttf::draw_text, the cached chrome path) and fill_rect
//! stay bit-identical to the build that blended glyphs in floats.

use nonos_toolkit::font::ttf;
use nonos_toolkit::paint::PaintBuffer;

use super::serial::serial;

const W: u32 = 320;
const H: u32 = 200;

/* FNV-1a of `scene()`. It was the float-blend toolkit's (commit 0080d463f)
until GPOS pair kerning moved the pen between glyphs of the built-in faces;
with that kerning turned off the scene still hashes to 0x67c6_aa75_9eb5_331b,
so the blend and the fills are unchanged since the float build. */
const FLOAT_BUILD: u64 = 0x25ae_e13b_2aee_14e7;

/* Chrome text over a background that walks every channel value, in opaque
and translucent colours, both built-in faces, clipped at all four edges,
then fills that run off the surface. */
fn scene() -> Vec<u32> {
    let mut buf: Vec<u32> = (0..W * H)
        .map(|i| {
            let v = (i % W * 3 + i / W * 7) & 0xff;
            0xff00_0000 | v << 16 | ((v * 5) & 0xff) << 8 | (255 - v)
        })
        .collect();
    let colors = [0xff00_0000u32, 0xffff_ffff, 0xff33_66cc, 0x80ff_ffff, 0xc010_2030, 0x40ff_8800];
    let sizes = [8.0f32, 17.0, 18.5, 22.0, 31.0];
    let text = "Hamburgefonstiv AVfi 0123 \u{e9}\u{3b1}\u{416}";
    let mut y = -9;
    for (i, &c) in colors.iter().enumerate() {
        for (j, &px) in sizes.iter().enumerate() {
            let x = -7 + ((i * 5 + j) as i32 * 13) % 41;
            let (top, mono) = (y + j as i32 * 6, (i + j) % 2 == 1);
            let end = ttf::draw_text(&mut buf, W as usize, W, H, x, top, text, c, px, mono);
            buf[(i * 5 + j) % (W * H) as usize] ^= end as u32;
        }
        y += 34;
    }
    let mut pb = PaintBuffer { pixels: &mut buf, stride_words: W, width: W, height: H };
    pb.fill_rect(W - 4, H - 3, 30, 30, 0xff12_3456);
    pb.fill_rect(250, 2, 200, 3, 0xff00_ff00);
    pb.fill_rect(5, 5, 0, 9, 0xffff_0000);
    buf
}

#[test]
fn chrome_text_is_bit_identical_to_the_float_blend_build() {
    let _one = serial();
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for b in scene().iter().flat_map(|p| p.to_le_bytes()) {
        h = (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3);
    }
    assert_eq!(h, FLOAT_BUILD, "chrome text or fills changed: {h:#018x}");
}
