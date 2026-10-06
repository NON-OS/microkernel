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

//! The glyph cache's byte budget is a bound on real heap: a cache filled
//! and churned by text of many sizes holds no more than it accounts for,
//! whether its entries are body glyphs or tiny ones the tree itself outweighs.

use nonos_toolkit::font::ttf::{builtin_face, clear_glyph_cache, draw_text_tracked};
use nonos_toolkit::font::ttf::{glyph_cache_bytes, GLYPH_CACHE_BUDGET};

use super::counting::measure;
use super::serial::serial;

/* Draw `runs` lines at sizes from `px0` up by `step`, alternating bold and
regular; returns the heap still held afterwards and the bytes accounted. */
fn fill(runs: usize, px0: f32, step: f32) -> (usize, usize) {
    let (regular, bold) = (builtin_face(false, false).unwrap(), builtin_face(false, true).unwrap());
    let (w, h) = (1200u32, 900u32);
    let mut buf = vec![0xffff_ffffu32; (w * h) as usize];
    let text = "Sphinx of black quartz, judge my vow 0123456789 AVfiW";
    clear_glyph_cache();
    let (_, heap) = measure(|| {
        for i in 0..runs {
            let (f, px) = (if i % 3 == 0 { bold } else { regular }, px0 + i as f32 * step);
            let y = (i * 11 % 850) as i32;
            draw_text_tracked(f, &mut buf, w as usize, w, h, 1, y, text, 0xff20_3040, px, 0.25);
        }
    });
    let accounted = glyph_cache_bytes();
    clear_glyph_cache();
    (heap.held, accounted)
}

#[test]
fn a_full_glyph_cache_holds_no_more_heap_than_it_accounts() {
    let _one = serial();
    for (runs, px0, step) in [(400, 8.0, 0.29), (3000, 1.0, 0.003)] {
        let (held, accounted) = fill(runs, px0, step);
        assert!(accounted > GLYPH_CACHE_BUDGET / 2, "{runs} runs never filled the cache");
        assert!(held <= accounted, "{held} heap bytes held against {accounted} accounted");
        assert!(held <= GLYPH_CACHE_BUDGET, "{held} heap bytes held");
    }
}
