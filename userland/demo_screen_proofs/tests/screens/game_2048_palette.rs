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

//! Every 2048 tile's digits read at WCAG AA, 4.5:1 or more, measured as the
//! Terminal's palette proof measures its colours; and the game-over line,
//! gold on its band of the ground, reads as well.

#[path = "../../../capsule_game_2048/src/game/colors.rs"]
mod colors;

const PAINT: &str = include_str!("../../../capsule_game_2048/src/game/paint.rs");
/// Every value a tile shows, 2 up past 2048, where the palette stops changing.
const VALUES: [u64; 12] = [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];

fn luminance(c: u32) -> f64 {
    let lin = |v: u32| {
        let s = v as f64 / 255.0;
        if s <= 0.03928 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * lin((c >> 16) & 0xFF) + 0.7152 * lin((c >> 8) & 0xFF) + 0.0722 * lin(c & 0xFF)
}

fn contrast(a: u32, b: u32) -> f64 {
    let (x, y) = (luminance(a), luminance(b));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

#[test]
fn every_tiles_digits_read_at_aa() {
    for v in VALUES {
        let r = contrast(colors::text_color(v), colors::tile_color(v));
        assert!(r >= 4.5, "tile {v}: {r:.2}:1");
    }
}

#[test]
fn the_digits_are_drawn_in_their_tiles_ink_and_game_over_on_the_ground() {
    let flat: String = PAINT.split_whitespace().collect();
    assert!(flat.contains("digits,text_color(v),2);"), "the digits use text_color");
    assert!(flat.contains("fb.fill_rect(ox+24,oy+240,312,44,BG);"), "a band under game over");
    assert!(PAINT.contains("const ACCENT: u32 = 0xFFEDC22E;"));
    assert!(contrast(0xFFEDC22E, colors::INK) >= 4.5, "gold on the ground");
}
