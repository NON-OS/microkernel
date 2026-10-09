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
//! A raster decoded before its box knew the image's size (no hint yet, so
//! at natural size) shrinks to the box once relayout notes it, and the
//! bytes it gave up are free for the next image.

use super::store_tests::bmp;
use crate::browser::image::{ingest, note_size, Store};

#[test]
fn a_late_hint_shrinks_the_raster_and_frees_its_bytes() {
    let mut s = Store::new();
    note_size(&mut s, "a", 0, 0);
    ingest(&mut s, "a", &bmp(1024, 1024, [9, 8, 7]));
    assert_eq!(s.ready("a").map(|d| (d.w, d.h)), Some((1024, 1024)), "no hint: natural size");
    note_size(&mut s, "a", 200, 100);
    let d = s.ready("a").expect("still resident");
    assert_eq!((d.w, d.h), (200, 200), "shrunk to cover the 200x100 box");
    assert!(d.px.iter().all(|&p| p == 0xFF09_0807), "the box filter keeps a flat colour");
    note_size(&mut s, "a", 120, 120);
    assert_eq!(s.ready("a").map(|d| d.w), Some(200), "a smaller box never shrinks it further");
    /* 3 x 4 MiB + 4,000,000 bytes + a's 160,000 fit the 16 MiB budget only
     * if a's former 4 MiB went back to it. */
    for (i, (u, side)) in [("b", 1024), ("c", 1024), ("d", 1024), ("e", 1000)].iter().enumerate() {
        note_size(&mut s, u, *side, *side);
        ingest(&mut s, u, &bmp(*side, *side, [i as u8, 1, 2]));
    }
    assert!(s.ready("a").is_some(), "a was not evicted: its bytes were returned");
    assert!(["b", "c", "d", "e"].iter().all(|u| s.ready(u).is_some()));
}
