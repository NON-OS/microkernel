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

use super::dmar_rmrr::{parse_rmrr, rmrr_span};
use super::dmar_rmrr_names::{rmrr_barred_by, rmrr_own};
use super::rmrr_tests::{rmrr, scope};
use super::scope::SCOPE_ENDPOINT;

fn none(_: u8, _: u8, _: u8) -> Option<(u8, u8)> {
    None
}

#[test]
fn a_malformed_rmrr_still_bars_the_memory_it_names() {
    // A limit written as an end, not the last byte: parse refuses it.
    let bad = rmrr(0, 0x7A00_0000, 0x7A02_0000, &[scope(SCOPE_ENDPOINT, 0, &[(0x1d, 0)])]);
    assert!(parse_rmrr(&bad).is_none());
    assert_eq!(rmrr_span(&bad), Some((0x7A00_0000, 0x7A02_0FFF)), "rounded outward");
    let inverted = rmrr(0, 0x7A02_0000, 0x7A00_0000, &[]);
    assert_eq!(rmrr_span(&inverted), Some((0x7A00_0000, 0x7A02_0FFF)));
    assert_eq!(rmrr_span(&inverted[..20]), None, "a header cut short names nothing");
    let ok = rmrr(0, 0x7A00_0000, 0x7A01_FFFF, &[scope(SCOPE_ENDPOINT, 0, &[(0x14, 0)])]);
    let ok = parse_rmrr(&ok).expect("parsed");
    let span = rmrr_span(&bad).expect("span");
    assert!(rmrr_barred_by(&[span], &ok), "the kept one shares memory a dropped one names");
    assert!(!rmrr_barred_by(&[(0x7A02_0000, 0x7A02_FFFF)], &ok), "memory apart bars nothing");
}

#[test]
fn a_region_another_segment_names_too_is_not_the_device_s_own() {
    let ep = || [scope(SCOPE_ENDPOINT, 0, &[(0x14, 0)])];
    let on = |seg| parse_rmrr(&rmrr(seg, 0x7A00_0000, 0x7A01_FFFF, &ep())).expect("parsed");
    let (r, o) = (on(0), on(1));
    assert!(rmrr_own(&[r], &r, 0, (0, 0x14, 0), &none));
    assert!(!rmrr_own(&[r, o], &r, 0, (0, 0x14, 0), &none), "the same bytes, another segment");
}
