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

//! The image store under memory pressure: rasters past the 16 MiB budget
//! push out the least recently painted ones instead of failing, and an
//! image larger than the budget decodes smaller rather than not at all.

use crate::browser::image::{ingest, note_size, Store};

const BUDGET: usize = 16 * 1024 * 1024;

/// A w x h 32-bit BMP, the kind the toolkit decoder reads, one colour.
pub(super) fn bmp(w: u32, h: u32, rgb: [u8; 3]) -> Vec<u8> {
    let size = 54 + 4 * (w * h) as usize;
    let mut b = Vec::with_capacity(size);
    b.extend_from_slice(b"BM");
    for v in [size as u32, 0, 54, 40, w, h] {
        b.extend_from_slice(&v.to_le_bytes());
    }
    b.extend_from_slice(&[1, 0, 32, 0]);
    b.extend_from_slice(&[0u8; 24]);
    b.extend_from_slice(&[rgb[2], rgb[1], rgb[0], 0xff].repeat((w * h) as usize));
    b
}

fn resident(s: &Store, urls: &[&str]) -> usize {
    urls.iter().filter_map(|u| s.ready(u)).map(|d| d.px.len() * 4).sum()
}

#[test]
fn the_least_recently_painted_raster_is_evicted_for_a_new_one() {
    let mut s = Store::new();
    let urls = ["a", "b", "c", "d", "e"];
    for (i, url) in urls.iter().enumerate() {
        note_size(&mut s, url, 1024, 1024);
        ingest(&mut s, url, &bmp(1024, 1024, [i as u8 * 40, 90, 200]));
        assert!(s.ready(url).is_some(), "{url} must decode, not fail for budget");
        assert!(resident(&s, &urls) <= BUDGET);
        if *url == "d" {
            s.ready("a").expect("a is still resident before e arrives");
        }
    }
    assert!(s.ready("b").is_none(), "b was painted least recently");
    assert!(["a", "c", "d", "e"].iter().all(|u| s.ready(u).is_some()), "the rest stay");
    assert!(s.contains("b"), "an evicted image stays known: only a box on screen revives it");
    assert_eq!(s.natural("b"), Some((1024, 1024)), "an evicted image keeps its layout size");
}

#[test]
fn an_image_past_the_whole_budget_decodes_smaller() {
    let mut s = Store::new();
    note_size(&mut s, "small", 1024, 1024);
    ingest(&mut s, "small", &bmp(1024, 1024, [1, 2, 3]));
    note_size(&mut s, "big", 4096, 2304);
    ingest(&mut s, "big", &super::jpeg_tests::fixture("big_prog.jpg"));
    let d = s.ready("big").expect("decoded, not refused");
    assert!(d.px.len() * 4 <= BUDGET, "{}x{} is past the budget", d.w, d.h);
    assert!(d.w < 4096 && (d.w as f64 / d.h as f64 - 16.0 / 9.0).abs() < 0.01, "{}x{}", d.w, d.h);
    assert!(s.ready("small").is_none(), "the older raster made room");
    assert_eq!(s.natural("big"), Some((4096, 2304)));
}
