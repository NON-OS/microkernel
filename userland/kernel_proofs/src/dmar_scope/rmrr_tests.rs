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

use super::dmar_rmrr::{parse_rmrr, RMRR_TYPE};
use super::scope::SCOPE_ENDPOINT;

/* An RMRR: type 1, length, reserved, segment, base, limit, then scopes. */
pub(super) fn rmrr(segment: u16, base: u64, limit: u64, scopes: &[Vec<u8>]) -> Vec<u8> {
    let mut b = vec![1u8, 0, 0, 0, 0, 0];
    b.extend_from_slice(&segment.to_le_bytes());
    b.extend_from_slice(&base.to_le_bytes());
    b.extend_from_slice(&limit.to_le_bytes());
    for s in scopes {
        b.extend_from_slice(s);
    }
    let len = b.len() as u16;
    b[2..4].copy_from_slice(&len.to_le_bytes());
    b
}

pub(super) fn scope(kind: u8, start_bus: u8, path: &[(u8, u8)]) -> Vec<u8> {
    let mut s = vec![kind, (6 + 2 * path.len()) as u8, 0, 0, 0, start_bus];
    for &(d, f) in path {
        s.extend_from_slice(&[d, f]);
    }
    s
}

#[test]
fn a_usb_controllers_region_parses_with_its_device() {
    let b = rmrr(0, 0x7A00_0000, 0x7A01_FFFF, &[scope(SCOPE_ENDPOINT, 0, &[(0x14, 0)])]);
    let r = parse_rmrr(&b).expect("parsed");
    assert_eq!(
        (r.segment, r.base, r.limit, r.count, r.truncated),
        (0, 0x7A00_0000, 0x7A01_FFFF, 1, false)
    );
    assert_eq!(
        (r.scopes[0].start_bus, r.scopes[0].path[0], r.scopes[0].path_len),
        (0, (0x14, 0), 1)
    );
    assert_eq!(RMRR_TYPE, 1);
}

#[test]
fn a_region_that_is_not_whole_pages_or_is_inverted_is_left_out() {
    assert!(parse_rmrr(&rmrr(0, 0x1000_0800, 0x1000_FFFF, &[])).is_none());
    assert!(parse_rmrr(&rmrr(0, 0x1000_0000, 0x1000_F000, &[])).is_none());
    assert!(parse_rmrr(&rmrr(0, 0x2000_0000, 0x1000_0FFF, &[])).is_none());
    assert!(parse_rmrr(&rmrr(0, 0, 0xFFF, &[])).is_some(), "one page at zero is a region");
}
