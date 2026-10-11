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

//! The device id table the driver claims by.

use crate::ids::{all, family, Family, VENDOR_INTEL};

#[test]
fn each_family_has_its_ids_and_another_vendor_has_none() {
    assert_eq!(family(VENDOR_INTEL, 0x1916), Some(Family::Skylake));
    assert_eq!(family(VENDOR_INTEL, 0x5917), Some(Family::KabyLake));
    assert_eq!(family(VENDOR_INTEL, 0x3E9B), Some(Family::CoffeeLake));
    assert_eq!(family(VENDOR_INTEL, 0x9BC4), Some(Family::CometLake));
    assert_eq!(family(0x1002, 0x1916), None);
    assert_eq!(family(VENDOR_INTEL, 0x5A85), None, "Apollo Lake waits for its unit");
    assert_eq!(family(VENDOR_INTEL, 0x8A52), None, "Ice Lake is unit 9");
}

#[test]
fn the_whole_file_was_read() {
    assert_eq!(all().len(), 88);
    let count = |f| all().iter().filter(|e| e.1 == f).count();
    assert_eq!((count(Family::Skylake), count(Family::KabyLake)), (25, 23));
    assert_eq!((count(Family::CoffeeLake), count(Family::CometLake)), (21, 19));
}
