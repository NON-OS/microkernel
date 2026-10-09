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

//! What the ID table leaves out, the ULP exemptions, and the family order.

use crate::constants::ids::*;
use crate::constants::Family;

#[test]
fn ids_of_other_drivers_are_not_claimed() {
    // e1000 (82540EM), igb (I210, I211), igc (I225), 82579 (pch2lan), 82577.
    for id in [0x100E, 0x1533, 0x1539, 0x15F3, 0x1502, 0x10EA, 0x0000, 0xFFFF] {
        assert_eq!(Family::of(id), None, "{id:#06x} is not an e1000e part here");
    }
}

#[test]
fn the_ulp_exceptions_are_lpt_parts() {
    for &id in NO_ULP {
        assert_eq!(Family::of(id), Some(Family::PchLpt));
    }
}

#[test]
fn families_order_as_linux_mac_types() {
    let order: Vec<Family> = super::ids_tests::BOARDS.iter().map(|b| b.1).collect();
    assert!(order.windows(2).all(|w| w[0] < w[1]));
}
