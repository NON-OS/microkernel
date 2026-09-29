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

/* Gradient mask-image: the value is kept by id with only its gradient
 * layers, the coverage follows the gradient inside the box and is 0 past
 * its edge, layers intersect or add, and a pixel shows old and new mixed
 * by that coverage. */
use crate::browser::layout::fade_table::{fade_id, fade_value};
use crate::browser::paint::{mask_row, mix};

const DOWN: &str = "linear-gradient(to bottom, #000 50%, transparent 100%)";
const RIGHT: &str = "linear-gradient(to right, transparent 0%, #000 50%)";

#[test]
fn a_mask_value_keeps_its_gradient_layers_only() {
    let id = fade_id("linear-gradient(red, blue) center / cover no-repeat");
    assert_eq!(fade_value(id).as_deref(), Some("linear-gradient(red, blue)"));
    assert_eq!(fade_id(DOWN), fade_id(DOWN), "one value, one id");
    assert_eq!(fade_id("none"), 0);
    assert_eq!(fade_id("url(m.svg), linear-gradient(red, blue)"), 0, "a url layer is not a fade");
}

#[test]
fn coverage_follows_the_gradient_and_stops_at_the_box() {
    let o = [10, 0, 20, 100];
    let top = mask_row(DOWN, o, false, (10, 0), 40);
    assert!(top[..10].iter().chain(&top[30..]).all(|&m| m == 0), "outside the box");
    assert!(top[10..30].iter().all(|&m| m == 255), "fully shown above 50%");
    let mid = mask_row(DOWN, o, false, (75, 0), 40)[20];
    assert!((120..=135).contains(&mid), "half way down the fade: {mid}");
    assert!(mask_row(DOWN, o, false, (99, 0), 40)[20] <= 8);
    assert!(mask_row(DOWN, o, false, (100, 0), 40).iter().all(|&m| m == 0), "below the box");
}

#[test]
fn layers_intersect_or_add() {
    let both = format!("{DOWN}, {RIGHT}");
    let o = [0, 0, 100, 100];
    let isect = mask_row(&both, o, true, (10, 0), 100);
    let add = mask_row(&both, o, false, (10, 0), 100);
    assert!(isect[0] <= 3 && add[0] >= 252, "left edge: hidden by one, shown by the other");
    assert!(isect[80] >= 252 && add[80] >= 252);
}

#[test]
fn a_pixel_mixes_old_and_new_by_coverage() {
    let (old, new) = (0xff00_0000, 0xffff_ffff);
    assert_eq!(mix(old, new, 0), old);
    assert_eq!(mix(old, new, 255), new);
    assert_eq!(mix(old, new, 128), 0xff80_8080);
}
