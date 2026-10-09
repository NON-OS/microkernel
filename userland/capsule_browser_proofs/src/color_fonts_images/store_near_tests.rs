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

/* Eviction under a burst of arrivals: rasters whose box is away from the
 * screen go before one on screen, even when the on-screen one was painted
 * longer ago than they were decoded. */

use super::store_tests::bmp;
use crate::browser::image::{ingest, note_size, Store};

#[test]
fn an_on_screen_raster_outlives_newer_off_screen_ones() {
    let mut s = Store::new();
    for (i, url) in ["hero", "far1", "far2", "far3", "far4"].iter().enumerate() {
        note_size(&mut s, url, 1024, 1024);
        if *url == "hero" {
            s.note_near(url);
        }
        ingest(&mut s, url, &bmp(1024, 1024, [i as u8 * 40, 90, 200]));
    }
    assert!(s.ready("hero").is_some(), "the on-screen raster was evicted for an off-screen one");
    assert!(s.ready("far1").is_none(), "the oldest off-screen raster made room");
    assert!(["far2", "far3", "far4"].iter().all(|u| s.ready(u).is_some()));
}

#[test]
fn with_only_on_screen_rasters_left_the_least_recent_goes() {
    let mut s = Store::new();
    for url in ["a", "b", "c", "d", "e"] {
        note_size(&mut s, url, 1024, 1024);
        s.note_near(url);
        ingest(&mut s, url, &bmp(1024, 1024, [9, 90, 200]));
    }
    assert!(s.ready("a").is_none(), "a was painted least recently");
    assert!(["b", "c", "d", "e"].iter().all(|u| s.ready(u).is_some()));
}
