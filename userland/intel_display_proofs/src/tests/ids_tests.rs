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

//! The device id tables the driver claims by.

use crate::engine::gen::{GEN11, GEN12, GEN12_3, GEN7_5, GEN8, GEN9};
use crate::ids::{all, family, Family, VENDOR_INTEL};

#[test]
fn each_family_has_its_ids_and_another_vendor_has_none() {
    assert_eq!(family(VENDOR_INTEL, 0x0416), Some(Family::Haswell));
    assert_eq!(family(VENDOR_INTEL, 0x1616), Some(Family::Broadwell));
    assert_eq!(family(VENDOR_INTEL, 0x1916), Some(Family::Skylake));
    assert_eq!(family(VENDOR_INTEL, 0x5917), Some(Family::KabyLake));
    assert_eq!(family(VENDOR_INTEL, 0x3E9B), Some(Family::CoffeeLake));
    assert_eq!(family(VENDOR_INTEL, 0x9BC4), Some(Family::CometLake));
    assert_eq!(family(VENDOR_INTEL, 0x8A52), Some(Family::IceLake));
    assert_eq!(family(VENDOR_INTEL, 0x9A49), Some(Family::TigerLake));
    assert_eq!(family(VENDOR_INTEL, 0x4C8A), Some(Family::RocketLake));
    assert_eq!(family(VENDOR_INTEL, 0x4680), Some(Family::AlderLakeS));
    assert_eq!(family(VENDOR_INTEL, 0x46A6), Some(Family::AlderLakeP));
    assert_eq!(family(VENDOR_INTEL, 0xA780), Some(Family::RaptorLakeS));
    assert_eq!(family(VENDOR_INTEL, 0xA7A0), Some(Family::RaptorLakeP));
    assert_eq!(family(0x1002, 0x1916), None);
    assert_eq!(family(VENDOR_INTEL, 0x5A85), None, "Apollo Lake waits for its unit");
    assert_eq!(family(VENDOR_INTEL, 0x22B0), None, "Cherry View waits for its unit");
}

#[test]
fn every_file_was_read_and_no_id_is_listed_twice() {
    let count = |f| all().filter(|e| e.1 == f).count();
    assert_eq!(all().count(), 45 + 18 + 88 + 13 + 57);
    assert_eq!((count(Family::Haswell), count(Family::Broadwell)), (45, 18));
    assert_eq!((count(Family::Skylake), count(Family::KabyLake)), (25, 23));
    assert_eq!((count(Family::CoffeeLake), count(Family::CometLake)), (21, 19));
    assert_eq!((count(Family::IceLake), count(Family::TigerLake)), (13, 11));
    assert_eq!((count(Family::RocketLake), count(Family::AlderLakeS)), (6, 8));
    assert_eq!((count(Family::AlderLakeP), count(Family::RaptorLakeS)), (18, 8));
    assert_eq!(count(Family::RaptorLakeP), 6);
    let ids: Vec<u16> = all().map(|e| e.0).collect();
    let mut sorted = ids.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "an id in two files");
}

#[test]
fn each_family_carries_its_generation_and_only_gen9_sets_modes() {
    assert_eq!(*Family::Haswell.gen(), GEN7_5);
    assert_eq!(*Family::Broadwell.gen(), GEN8);
    assert_eq!(*Family::CometLake.gen(), GEN9);
    assert_eq!(*Family::IceLake.gen(), GEN11);
    assert_eq!(*Family::RocketLake.gen(), GEN12_3);
    assert_eq!(*Family::RaptorLakeP.gen(), GEN12);
    let setters: Vec<Family> = all().map(|e| e.1).filter(|f| f.sets_modes()).collect();
    assert_eq!(setters.len(), 88);
    assert!(!Family::TigerLake.sets_modes() && !Family::Broadwell.sets_modes());
}
