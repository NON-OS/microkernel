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

use super::behaviour::zero_length_read;

const MAPPING: &str = include_str!("../../../../src/arch/x86_64/iommu/backend/mapping.rs");

#[test]
fn zlr_is_capability_bit_twenty_two() {
    assert!(zero_length_read(1 << 22));
    assert!(!zero_length_read(!(1u64 << 22)));
}

#[test]
fn a_write_only_grant_keeps_read_on_a_unit_without_zlr() {
    // The VT-d backend folds the rule into the leaf's read bit; a refactor
    // that drops it would fault devices that flush with a zero-length read.
    assert!(MAPPING.contains("read: protection.read || (protection.write && !zlr),"));
}
