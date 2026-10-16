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

use super::dmar_rmrr::{parse_rmrr, Rmrr};
use super::dmar_rmrr_names::{rmrr_own, rmrr_sole};
use super::rmrr_tests::{rmrr, scope};
use super::scope::SCOPE_ENDPOINT;

const NAMESPACE: u8 = 5;
fn none(_: u8, _: u8, _: u8) -> Option<(u8, u8)> {
    None
}
fn one(base: u64, limit: u64, scopes: &[Vec<u8>]) -> Rmrr {
    parse_rmrr(&rmrr(0, base, limit, scopes)).expect("parsed")
}

#[test]
fn a_device_off_pci_beside_the_endpoint_makes_the_region_shared() {
    let ep = scope(SCOPE_ENDPOINT, 0, &[(0x15, 0)]);
    let r = one(0x7A00_0000, 0x7A01_FFFF, &[ep.clone(), scope(NAMESPACE, 0, &[(0x15, 1)])]);
    assert_eq!((r.count, r.truncated, r.foreign), (1, false, true));
    assert!(!rmrr_sole(&r), "the namespace device keeps using it outside any capsule");
    let alone = one(0x7A00_0000, 0x7A01_FFFF, &[ep]);
    assert!(!alone.foreign && rmrr_sole(&alone));
}

#[test]
fn a_region_two_rmrrs_give_two_devices_is_neither_s_own() {
    let (a, b) = ((0, 0x14, 0), (0, 0x1d, 0));
    let to = |dev: u8| [scope(SCOPE_ENDPOINT, 0, &[(dev, 0)])];
    let r1 = one(0x7A00_0000, 0x7A01_FFFF, &to(0x14));
    let r2 = one(0x7A01_0000, 0x7A02_FFFF, &to(0x1d));
    let all = [r1, r2];
    assert!(rmrr_sole(&r1) && rmrr_sole(&r2), "each alone looks sole");
    assert!(!rmrr_own(&all, &r1, 0, a, &none) && !rmrr_own(&all, &r2, 0, b, &none));
    // The same region listed twice for the one device stays its own.
    let twice = [r1, r1];
    assert!(rmrr_own(&twice, &r1, 0, a, &none));
    // A region only touching another's last page is still shared.
    let next = one(0x7A01_F000, 0x7A03_FFFF, &to(0x1d));
    assert!(!rmrr_own(&[r1, next], &r1, 0, a, &none));
    let apart = one(0x7A02_0000, 0x7A03_FFFF, &to(0x1d));
    assert!(rmrr_own(&[r1, apart], &r1, 0, a, &none), "regions apart are each their own");
}

#[test]
fn a_sole_region_inside_a_shared_one_is_not_the_device_s_own() {
    let gpu = one(0x7B80_0000, 0x7B8F_FFFF, &[scope(SCOPE_ENDPOINT, 0, &[(0x02, 0)])]);
    let wide = [scope(SCOPE_ENDPOINT, 0, &[(0x02, 0)]), scope(SCOPE_ENDPOINT, 0, &[(0x14, 0)])];
    let wide = one(0x7B00_0000, 0x7BFF_FFFF, &wide);
    assert!(!rmrr_own(&[gpu, wide], &gpu, 0, (0, 0x02, 0), &none));
}
