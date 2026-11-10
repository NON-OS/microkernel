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

//! The mono helpers draw, place and cut against one fixed advance per char.

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{mono_left, mono_right, mono_width, truncate_mono};

use super::{boxes::boxed, canvas::Canvas};
use super::samples::{sizes, RUNS};
use super::serial::serial;

#[test]
fn mono_width_is_the_pen_advance_and_every_cell_is_alike() {
    let _g = serial();
    let (at, ink) = (boxed(900, 80), roles().text);
    for px in sizes() {
        for s in ["0123456789", "iiiiiWWWWW", "ff:0a:9c", ""] {
            let mut pen = 0;
            let _ = Canvas::new(at, 0).painted(|fb| pen = mono_left(fb, at.x, 30, s, ink, px));
            assert_eq!(i64::from(mono_width(s, px)), i64::from(pen) - i64::from(at.x), "{s:?}");
            assert_eq!(mono_width(s, px), mono_width(&"0".repeat(s.len()), px), "{s:?} at {px}");
        }
    }
}

#[test]
fn mono_right_ends_the_run_on_the_edge() {
    let _g = serial();
    let (at, ink) = (boxed(900, 80), roles().text);
    let r = at.x + 400;
    for px in sizes() {
        for s in ["7", "1024", "65535 KiB"] {
            let (mut x, mut pen) = (0, 0);
            let _ = Canvas::new(at, 0).painted(|fb| {
                x = mono_right(fb, r, 30, s, ink, px);
                pen = mono_left(fb, x, 30, s, ink, px);
            });
            assert_eq!((x, pen), (r - mono_width(s, px), r as i32), "{s:?} at {px}");
        }
    }
}

#[test]
fn truncate_mono_fits_the_mono_advance_on_a_char_boundary() {
    let _g = serial();
    for px in sizes() {
        for s in RUNS.into_iter().chain(["0x0000_7fff_ffff_e000"]) {
            for max_w in 0..=mono_width(s, px) + 4 {
                let out = truncate_mono(s, max_w, px);
                let why = format!("{s:?} at {px} in {max_w}: {out:?}");
                assert!(mono_width(&out, px) <= max_w, "{why} is too wide");
                let stem = out.strip_suffix('…').unwrap_or(&out);
                assert!(s.starts_with(stem) && s.is_char_boundary(stem.len()), "{why}");
                let full = mono_width(s, px) <= max_w;
                assert_eq!(full, out == s, "{why} cut a run that fits, or kept one that does not");
                let next = s[stem.len()..].chars().next().map(|c| format!("{stem}{c}…"));
                assert!(full || next.is_none_or(|n| mono_width(&n, px) > max_w), "{why} short");
            }
        }
    }
}
