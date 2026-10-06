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
//! Vector art drawn only in an <img> box rasterizes at the box's own shape,
//! with the art's preserveAspectRatio placing it; a background keeps the
//! art's natural shape for background-size to work with.

use crate::browser::image::{ingest, note_img_size, note_size, Store};

fn svg(par: &str) -> Vec<u8> {
    format!(
        "<svg xmlns='http://www.w3.org/2000/svg' width='100' height='50' viewBox='0 0 100 50' \
         preserveAspectRatio='{par}'><rect width='100' height='50' fill='red'/></svg>"
    )
    .into_bytes()
}

fn at(s: &Store, u: &str, x: u32, y: u32) -> u32 {
    let d = s.ready(u).expect("decoded");
    d.px[(y * d.w + x) as usize]
}

#[test]
fn an_img_box_gets_a_raster_of_its_own_shape() {
    let mut s = Store::new();
    for (u, par) in [("none", "none"), ("meet", "xMidYMid meet"), ("slice", "xMinYMin slice")] {
        note_img_size(&mut s, u, 400, 300);
        ingest(&mut s, u, &svg(par));
        assert_eq!(s.ready(u).map(|d| (d.w, d.h)), Some((400, 300)), "{u}");
        assert_eq!(s.natural(u), Some((100, 50)), "{u}: the natural size is unchanged");
    }
    assert_eq!(at(&s, "none", 0, 2), 0xFFFF_0000, "none stretches over the whole box");
    assert_eq!(at(&s, "meet", 200, 10), 0, "meet letterboxes above the art");
    assert_eq!(at(&s, "meet", 200, 150), 0xFFFF_0000);
    assert_eq!(at(&s, "slice", 399, 299), 0xFFFF_0000, "slice covers the box");
    note_size(&mut s, "bg", 400, 300);
    ingest(&mut s, "bg", &svg("none"));
    let d = s.ready("bg").expect("decoded");
    assert_eq!((d.w, d.h), (600, 300), "not an <img> box: the natural 2:1 shape covering it");
}
