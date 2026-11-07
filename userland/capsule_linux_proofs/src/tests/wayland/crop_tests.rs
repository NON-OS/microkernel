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

//! The window geometry's box of a buffer (wayland/present/crop.rs), and a
//! change said in buffer pixels moved to the box's corner (damage.rs).

use crate::damage::Damage;
use crate::geometry_crop::{corner, crop, crop_dest, span};

#[test]
fn firefoxs_shadow_is_cut_to_its_window() {
    let g = Some((26, 23, 800, 600));
    assert_eq!(crop(g, 1, (852, 652)), Some((26, 23, 800, 600)));
    // At buffer scale 2 the geometry is in logical pixels.
    assert_eq!(crop(g, 2, (1704, 1304)), Some((52, 46, 1600, 1200)));
}

#[test]
fn a_geometry_that_is_the_buffer_or_none_of_it_shows_the_buffer_whole() {
    assert_eq!(crop(None, 1, (800, 600)), None);
    assert_eq!(crop(Some((0, 0, 800, 600)), 1, (800, 600)), None, "the whole buffer");
    assert_eq!(crop(Some((0, 0, 900, 700)), 1, (800, 600)), None, "past it, cut to the whole");
    assert_eq!(crop(Some((800, 0, 10, 10)), 1, (800, 600)), None, "beside it");
    assert_eq!(crop(Some((-10, -10, 5, 5)), 1, (800, 600)), None, "before it");
    assert_eq!(crop(Some((i32::MAX, 0, i32::MAX, 1)), 4, (800, 600)), None, "no wrap");
}

#[test]
fn a_geometry_past_the_buffer_is_cut_to_it() {
    assert_eq!(crop(Some((-4, 10, 100, 1000)), 1, (800, 600)), Some((0, 10, 96, 590)));
    assert_eq!(crop(Some((700, 500, 400, 400)), 1, (800, 600)), Some((700, 500, 100, 100)));
}

#[test]
fn the_read_ends_at_the_boxs_last_pixel_never_past_the_buffer() {
    let stride = 852 * 4;
    let (start, len) = span((26, 23, 800, 600), stride);
    assert_eq!(start, 23 * u64::from(stride) + 26 * 4);
    assert_eq!(len, 599 * stride as usize + 800 * 4);
    // A box at the buffer's bottom right ends where the buffer does.
    let (start, len) = span((52, 52, 800, 600), stride);
    assert_eq!(start + len as u64, 652 * u64::from(stride));
}

#[test]
fn a_change_is_moved_to_the_windows_corner() {
    let said = Damage::Rect(36, 53, 56, 61);
    assert_eq!(said.moved((26, 23)), Damage::Rect(10, 30, 30, 38));
    assert_eq!(said.moved((26, 23)).on(800, 600), Damage::Rect(10, 30, 30, 38));
    // In the shadow alone: nothing of the window.
    assert_eq!(Damage::Rect(0, 0, 8, 8).moved((26, 23)).on(800, 600), Damage::None);
    assert_eq!(Damage::Whole.moved((26, 23)), Damage::Whole);
    assert_eq!(Damage::None.moved((26, 23)), Damage::None);
}

#[test]
fn a_popup_surface_starts_its_geometrys_corner_before_its_place() {
    assert_eq!(corner(Some((12, 12, 100, 50))), (12, 12), "a GTK menu's shadow");
    assert_eq!(corner(Some((26, 23, 800, 600))), (26, 23));
    assert_eq!(corner(None), (0, 0), "no geometry: the surface is the window");
    assert_eq!(corner(Some((-10, -4, 5, 5))), (0, 0), "never before the surface");
    assert_eq!(corner(Some((i32::MIN, i32::MAX, 1, 1))), (0, i32::MAX));
}

#[test]
fn with_a_viewport_the_geometry_is_in_its_destinations_pixels() {
    let g = Some((26, 23, 800, 600));
    // Telegram at 1.25: an 852x652 surface drawn 1065x815.
    assert_eq!(crop_dest(g, (852, 652), (1065, 815)), Some(((33, 29, 1000, 750), (800, 600))));
    // A destination of the buffer's own size crops as crop does at scale 1.
    assert_eq!(crop_dest(g, (852, 652), (852, 652)), Some(((26, 23, 800, 600), (800, 600))));
    // Past the surface's edge the box is what lies on it.
    let past = crop_dest(Some((26, 23, 900, 700)), (852, 652), (852, 652));
    assert_eq!(past, Some(((26, 23, 826, 629), (826, 629))));
}

#[test]
fn with_a_viewport_a_geometry_of_all_or_none_of_the_surface_shows_it_whole() {
    let (dest, buf) = ((852, 652), (1065, 815));
    assert_eq!(crop_dest(None, dest, buf), None);
    assert_eq!(crop_dest(Some((0, 0, 852, 652)), dest, buf), None, "the whole surface");
    assert_eq!(crop_dest(Some((-5, -5, 900, 700)), dest, buf), None, "past it, cut to the whole");
    assert_eq!(crop_dest(Some((852, 0, 10, 10)), dest, buf), None, "beside it");
    assert_eq!(crop_dest(Some((0, 0, 0, 10)), dest, buf), None, "no width");
    assert_eq!(crop_dest(Some((i32::MAX, 0, i32::MAX, 1)), dest, buf), None, "no wrap");
    assert_eq!(crop_dest(Some((i32::MIN, i32::MIN, i32::MAX, i32::MAX)), dest, buf), None);
    // A box narrower than a buffer pixel after rounding is none.
    assert_eq!(crop_dest(Some((10, 10, 1, 1)), (852, 652), (1, 1)), None);
}
