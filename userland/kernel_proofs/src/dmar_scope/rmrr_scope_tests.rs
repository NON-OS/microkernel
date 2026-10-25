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

use super::dmar_rmrr::{parse_rmrr, MAX_RMRR_SCOPES};
use super::rmrr_tests::{rmrr, scope};
use super::scope::SCOPE_BRIDGE;

#[test]
fn a_short_or_mistyped_structure_is_not_an_rmrr() {
    let good = rmrr(0, 0x1000, 0x1FFF, &[]);
    assert!(parse_rmrr(&good[..23]).is_none());
    let mut drhd = good.clone();
    drhd[0] = 0;
    assert!(parse_rmrr(&drhd).is_none());
}

#[test]
fn scopes_past_the_room_kept_mark_it_truncated_and_non_pci_ones_are_skipped() {
    let mut many: Vec<Vec<u8>> =
        (0..MAX_RMRR_SCOPES as u8 + 1).map(|d| scope(SCOPE_BRIDGE, 0, &[(d, 0)])).collect();
    let r = parse_rmrr(&rmrr(0, 0x1000, 0x1FFF, &many)).expect("parsed");
    assert_eq!((r.count as usize, r.truncated), (MAX_RMRR_SCOPES, true));
    many.truncate(1);
    many.push(scope(3, 0, &[(0x1e, 7)]));
    let r = parse_rmrr(&rmrr(0, 0x1000, 0x1FFF, &many)).expect("parsed");
    assert_eq!((r.count, r.truncated), (1, false), "an IOAPIC scope names no PCI device");
}
