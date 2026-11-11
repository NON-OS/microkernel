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

//! The process monitor's names and notes cut by the toolkit's `clip`, and its
//! numbers measured by the toolkit's `mono_width`, as its private `fit` and
//! `mono_width` did.

use nonos_toolkit::paint::{measure_ttf, measure_ttf_mono};
use nonos_toolkit::widgets::text::{clip, mono_width};

use crate::metrics::{BODY_PX, CARD_VALUE_PX, NUM_PX, TITLE_PX};
use crate::pm::ui::text::valid;

const NAMES: [&[u8]; 8] = [
    b"desktop_shell",
    b"capsule_process_manager",
    b"? keys",
    b"no processes match the filter",
    b"caf\xc3\xa9 \xc3\xbcn\xc3\xafcode \xe2\x9c\x93",
    b"1.2 GiB",
    b"\xff\xfe not utf-8",
    b"",
];

fn old_fit(bytes: &[u8], px: f32, max_w: u32) -> &[u8] {
    let s = valid(bytes);
    let mut end = s.len();
    while end > 0 {
        if measure_ttf(&s[..end], px).max(0) as u32 <= max_w {
            return &bytes[..end];
        }
        end -= 1;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
    }
    &bytes[..0]
}

#[test]
fn every_name_cuts_where_the_old_fit_cut_it() {
    for name in NAMES {
        for px in [BODY_PX, TITLE_PX] {
            let full = measure_ttf(valid(name), px).max(0) as u32;
            for max_w in (0..=full + 4).step_by(2) {
                let cut = clip(valid(name), max_w, px).as_bytes();
                assert_eq!(cut, old_fit(name, px, max_w), "{name:?} at {px} in {max_w}");
            }
        }
    }
}

#[test]
fn every_number_measures_as_the_old_mono_width_did() {
    for name in NAMES {
        for px in [NUM_PX, BODY_PX, CARD_VALUE_PX] {
            let old = measure_ttf_mono(valid(name), px).max(0) as u32;
            assert_eq!(mono_width(valid(name), px), old, "{name:?} at {px}");
        }
    }
}
