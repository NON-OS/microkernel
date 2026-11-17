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

//! About's mono cells, the licence columns, the verdict evidence and the
//! value rows, cut and measured by the toolkit's `truncate_mono` and
//! `mono_width` as About's private `truncate_mono` and `width_of_mono` did.

use nonos_toolkit::paint::measure_ttf_mono;
use nonos_toolkit::widgets::text::{mono_width, truncate_by, truncate_mono};

use crate::about::ui::metrics::metrics;

#[path = "../../capsule_about/src/about/data/third_party.rs"]
mod third_party;

const SAMPLES: [&[u8]; 8] = [
    b"0xffff_ffff_8000_0000",
    b"3f9a1c07d2e4b856",
    b"pid 4294967295",
    b"Ed25519 + ML-DSA-65 (hybrid)",
    b"caf\xc3\xa9 \xc3\xbcn\xc3\xafcode \xe2\x9c\x93",
    b"\xff\xfe not utf-8",
    b"a",
    b"",
];

fn valid(bytes: &[u8]) -> &str {
    core::str::from_utf8(bytes).unwrap_or("")
}

fn cells() -> impl Iterator<Item = &'static [u8]> {
    let table = third_party::COMPONENTS.iter().flat_map(|c| [c.name, c.role, c.license]);
    table.chain(SAMPLES)
}

fn old_truncate_mono(bytes: &[u8], max_w: u32, px: f32) -> String {
    let max_w = i32::try_from(max_w).unwrap_or(i32::MAX);
    truncate_by(valid(bytes), max_w, |t| measure_ttf_mono(t, px))
}

#[test]
fn every_mono_cell_cuts_where_the_old_truncate_mono_cut_it() {
    let m = metrics();
    for b in cells() {
        for px in [m.num_px, m.body_px] {
            let full = measure_ttf_mono(valid(b), px).max(0) as u32;
            for w in 0..=full + 4 {
                assert_eq!(truncate_mono(valid(b), w, px), old_truncate_mono(b, w, px), "{b:?} in {w}");
            }
        }
    }
}

#[test]
fn every_mono_cell_measures_as_the_old_width_of_mono_did() {
    let m = metrics();
    for b in cells() {
        for px in [m.num_px, m.body_px] {
            let old = measure_ttf_mono(valid(b), px).max(0) as u32;
            assert_eq!(mono_width(valid(b), px), old, "{b:?} at {px}");
        }
    }
}
