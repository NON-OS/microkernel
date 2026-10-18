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

//! an RMRR is believed only when exactly sound, and names only the
//! devices its scope resolves to; any doubt leaves a device with no
//! reserved range, which is the blocked side.

use super::build::{bridges, rmrr, scope, GFX};
use super::dmar_rmrr::{names, parse_rmrr, RMRR_MAX};

#[test]
fn a_sound_rmrr_names_its_devices_and_no_other() {
    let usb = scope(2, 0, &[(0x1c, 0)]);
    let b = rmrr(0x7c00_0000, 0x7fff_ffff, &[scope(1, 0, &[(2, 0)]), usb]);
    let r = parse_rmrr(&b).expect("sound");
    assert_eq!(
        (r.base, r.end, r.scope.count, r.scope.truncated),
        (0x7c00_0000, 0x8000_0000, 2, false)
    );
    assert!(names(&r, 0, GFX, &bridges), "the endpoint");
    assert!(names(&r, 0, (3, 0, 0), &bridges), "behind the named bridge");
    assert!(!names(&r, 0, (0, 3, 0), &bridges), "another device");
    assert!(!names(&r, 1, GFX, &bridges), "another segment");
}

#[test]
fn an_rmrr_that_is_not_exactly_sound_is_refused() {
    let s = || vec![scope(1, 0, &[(2, 0)])];
    assert!(parse_rmrr(&rmrr(0x1000, 0x1fff, &s())[..23]).is_none(), "short");
    let mut other = rmrr(0x1000, 0x1fff, &s());
    other[0] = 0;
    assert!(parse_rmrr(&other).is_none(), "a DRHD, not an RMRR");
    assert!(parse_rmrr(&rmrr(0x1001, 0x1fff, &s())).is_none(), "base off a page");
    assert!(parse_rmrr(&rmrr(0x1000, 0x1ffe, &s())).is_none(), "limit off a page");
    assert!(parse_rmrr(&rmrr(0x2000, 0x1fff, &s())).is_none(), "empty");
    assert!(parse_rmrr(&rmrr(0x3000, 0x1fff, &s())).is_none(), "limit below base");
    assert!(parse_rmrr(&rmrr(0, u64::MAX, &s())).is_none(), "the limit wraps");
    assert!(parse_rmrr(&rmrr(0, RMRR_MAX, &s())).is_none(), "over the cap");
    assert!(parse_rmrr(&rmrr(0, RMRR_MAX - 1, &s())).is_some(), "exactly the cap");
}

#[test]
fn a_hostile_scope_list_names_nothing_it_cannot_resolve() {
    let mut overrun = rmrr(0x1000, 0x1fff, &[scope(1, 0, &[(2, 0)])]);
    overrun[25] = 40; // the scope claims more bytes than the structure holds
    let r = parse_rmrr(&overrun).expect("the region still parses");
    assert!(r.scope.truncated && r.scope.count == 0);
    assert!(!names(&r, 0, GFX, &bridges));
    let lost = parse_rmrr(&rmrr(0x1000, 0x1fff, &[scope(1, 0, &[(9, 0), (0, 0)])])).expect("ok");
    assert!(!names(&lost, 0, (0, 0, 0), &bridges), "a path through no bridge resolves to nothing");
    for odd in [0u8, 5, 7] {
        let mut b = rmrr(0x1000, 0x1fff, &[scope(1, 0, &[(2, 0)])]);
        b[25] = odd;
        assert!(parse_rmrr(&b).is_some_and(|r| r.scope.truncated), "scope length {odd}");
    }
}
