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

//! A good plan, and each way a plan can be wrong, refused by name.

use super::plan::parse_plan;
use super::plan_types::{Plan, PlanError as E, DATA_FLOOR};

/*
 * A 2 GiB disk, in sectors.
 */
pub const DISK: u64 = 4 * 1024 * 1024;

pub fn plan(base: u64, sectors: u64, at: u64, bytes: u64) -> [u8; 512] {
    let mut s = [0u8; 512];
    s[..8].copy_from_slice(b"NONOSDP1");
    for (i, v) in [base, sectors, at, bytes].iter().enumerate() {
        s[8 + i * 8..16 + i * 8].copy_from_slice(&v.to_le_bytes());
    }
    s
}

#[test]
fn a_volume_and_an_import_that_fit_side_by_side_are_read_as_written() {
    let got = parse_plan(&plan(DATA_FLOOR, 1 << 20, DATA_FLOOR + (1 << 20), 491_400_032), DISK);
    let want = Plan {
        volume_base: DATA_FLOOR,
        volume_sectors: 1 << 20,
        import: Some((DATA_FLOOR + (1 << 20), 491_400_032)),
    };
    assert_eq!(got, Ok(want));
    assert_eq!(parse_plan(&plan(DATA_FLOOR, 1 << 20, 0, 0), DISK).unwrap().import, None);
}

#[test]
fn each_lie_a_plan_can_tell_is_refused_by_name() {
    assert_eq!(parse_plan(&[0u8; 512], DISK), Err(E::NoPlan));
    assert_eq!(parse_plan(&plan(256, 1 << 20, 0, 0), DISK), Err(E::BelowFloor));
    assert_eq!(parse_plan(&plan(DATA_FLOOR, 100, 0, 0), DISK), Err(E::VolumeTooSmall));
    assert_eq!(parse_plan(&plan(DATA_FLOOR, DISK, 0, 0), DISK), Err(E::PastEnd));
    assert_eq!(parse_plan(&plan(u64::MAX - 5, 1 << 20, 0, 0), DISK), Err(E::PastEnd));
    assert_eq!(parse_plan(&plan(DATA_FLOOR, 1 << 20, 256, 512), DISK), Err(E::BelowFloor));
    assert_eq!(parse_plan(&plan(DATA_FLOOR, 1 << 20, DISK - 1, 1024), DISK), Err(E::PastEnd));
    /*
     * An import that starts inside the volume, and one that ends inside it.
     */
    assert_eq!(parse_plan(&plan(DATA_FLOOR, 1 << 20, DATA_FLOOR + 5, 512), DISK), Err(E::Overlap));
    let before = DATA_FLOOR + 1000;
    assert_eq!(
        parse_plan(&plan(before + 10, 1 << 20, DATA_FLOOR + 1000, 20 * 512), DISK),
        Err(E::Overlap)
    );
    /*
     * Touching is not sharing: an import that begins on the volume's end sector.
     */
    assert!(parse_plan(&plan(DATA_FLOOR, 1 << 20, DATA_FLOOR + (1 << 20), 512), DISK).is_ok());
}
