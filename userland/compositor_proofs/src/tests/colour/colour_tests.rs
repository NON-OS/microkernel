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

//! What the settings do to levels: greyscale weighs light as BT.709 does,
//! high contrast only ever rises and keeps black, mid grey and white, and
//! the ramps are night light's then contrast's, or nothing at all.

use crate::display::access_rule::{Access, FILTER_GREYSCALE, FILTER_NONE};
use crate::display::colour_rule::{contrast, matrix, ramps, LUMA};

#[test]
fn greyscale_weighs_light_and_no_filter_is_none() {
    assert_eq!(matrix(FILTER_GREYSCALE), Some([LUMA; 3]));
    assert!((LUMA.iter().sum::<f32>() - 1.0).abs() < 1e-6, "white stays white");
    assert_eq!(matrix(FILTER_NONE), None);
}

#[test]
fn contrast_rises_and_keeps_its_three_points() {
    assert_eq!((contrast(0), contrast(128), contrast(255)), (0, 128, 255));
    assert!((1..=255u8).all(|v| contrast(v) >= contrast(v - 1)));
    assert!(contrast(160) > 160 && contrast(96) < 96, "pushed from mid grey");
}

#[test]
fn the_ramps_are_night_then_contrast() {
    let plain = Access::default();
    assert_eq!(ramps(&plain, None), None, "nothing to do");
    let mut night = [[0u8; 256]; 3];
    for (c, r) in night.iter_mut().enumerate() {
        for (v, x) in r.iter_mut().enumerate() {
            *x = (v / (c + 1)) as u8;
        }
    }
    assert_eq!(ramps(&plain, Some(&night)), Some(night));
    let sharp = Access { contrast: true, ..plain };
    let r = ramps(&sharp, Some(&night)).unwrap();
    assert!((0..256).all(|v| (0..3).all(|c| r[c][v] == contrast(night[c][v]))));
}
