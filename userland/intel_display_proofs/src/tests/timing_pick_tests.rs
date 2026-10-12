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

//! The timing a mode set lights a size at: the monitor's own detail for
//! its preferred mode, the DMT's for a mode it names by size and rate.

use super::edid_blob::edid_1080p_and_xga;
use crate::display::pick;

#[test]
fn the_preferred_mode_keeps_the_monitors_own_timing() {
    let e = edid_1080p_and_xga((600, 340));
    let t = pick(&e, 1920, 1080).unwrap();
    assert_eq!((t.h_total(), t.v_total(), t.pixel_khz), (2200, 1125, 148_500));
}

#[test]
fn an_established_mode_takes_the_dmt_timing() {
    let e = edid_1080p_and_xga((600, 340));
    let t = pick(&e, 1024, 768).unwrap();
    assert_eq!((t.h_total(), t.v_total(), t.pixel_khz), (1344, 806, 65_000));
}

#[test]
fn a_size_not_listed_or_a_blob_that_does_not_parse_has_none() {
    let e = edid_1080p_and_xga((600, 340));
    assert_eq!(pick(&e, 1280, 1024), None);
    assert_eq!(pick(&e[..64], 1920, 1080), None);
}
