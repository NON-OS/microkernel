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
use super::plan_types::{PlanError as E, DATA_FLOOR, MAX_IMPORTS};

/*
 * A 2 GiB disk, in sectors, and a volume of half a GiB at the floor.
 */
pub const DISK: u64 = 4 * 1024 * 1024;
pub const VOL: u64 = 1 << 20;
pub const AFTER: u64 = DATA_FLOOR + VOL;

pub fn plan(base: u64, sectors: u64, imports: &[(u64, u64)]) -> [u8; 512] {
    let mut s = [0u8; 512];
    s[..8].copy_from_slice(b"NONOSDP1");
    let head = [base, sectors, imports.len() as u64];
    let words = head.iter().chain(imports.iter().flat_map(|(a, b)| [a, b]));
    for (i, v) in words.enumerate() {
        s[8 + i * 8..16 + i * 8].copy_from_slice(&v.to_le_bytes());
    }
    s
}

#[test]
fn a_volume_and_its_imports_that_fit_side_by_side_are_read_as_written() {
    let two = [(AFTER, 491_400_032), (AFTER + 959_766, 689_872_288)];
    let got = parse_plan(&plan(DATA_FLOOR, VOL, &two), DISK).unwrap();
    assert_eq!((got.volume_base, got.volume_sectors), (DATA_FLOOR, VOL));
    assert_eq!(got.imports(), &two);
    assert!(parse_plan(&plan(DATA_FLOOR, VOL, &[]), DISK).unwrap().imports().is_empty());
}

#[test]
fn each_lie_a_plan_can_tell_is_refused_by_name() {
    let p = |b, s, i: &[(u64, u64)]| parse_plan(&plan(b, s, i), DISK).map(|_| ());
    assert_eq!(parse_plan(&[0u8; 512], DISK).map(|_| ()), Err(E::NoPlan));
    assert_eq!(p(256, VOL, &[]), Err(E::BelowFloor));
    assert_eq!(p(DATA_FLOOR, 100, &[]), Err(E::VolumeTooSmall));
    assert_eq!(p(DATA_FLOOR, DISK, &[]), Err(E::PastEnd));
    assert_eq!(p(u64::MAX - 5, VOL, &[]), Err(E::PastEnd));
    assert_eq!(p(DATA_FLOOR, VOL, &[(256, 512)]), Err(E::BelowFloor));
    assert_eq!(p(DATA_FLOOR, VOL, &[(DISK - 1, 1024)]), Err(E::PastEnd));
    assert_eq!(p(DATA_FLOOR, VOL, &[(AFTER, 0)]), Err(E::BadImport));
    let mut s = plan(DATA_FLOOR, VOL, &[]);
    s[24..32].copy_from_slice(&(MAX_IMPORTS as u64 + 1).to_le_bytes());
    assert_eq!(parse_plan(&s, DISK).map(|_| ()), Err(E::BadImport));
    /*
     * An import inside the volume, one ending inside it, and two imports
     * sharing a sector.
     */
    assert_eq!(p(DATA_FLOOR, VOL, &[(DATA_FLOOR + 5, 512)]), Err(E::Overlap));
    assert_eq!(p(DATA_FLOOR + 1010, VOL, &[(DATA_FLOOR + 1000, 20 * 512)]), Err(E::Overlap));
    assert_eq!(p(DATA_FLOOR, VOL, &[(AFTER, 1024), (AFTER + 1, 512)]), Err(E::Overlap));
    /*
     * Touching is not sharing: each range may begin on the last one's end.
     */
    assert!(p(DATA_FLOOR, VOL, &[(AFTER, 512), (AFTER + 1, 512)]).is_ok());
}

/*
 * A live stick's plan: no volume on the disk, only imports, each still
 * checked against the floor, the disk's end and every other import.
 */
#[test]
fn a_live_plan_names_imports_and_no_volume() {
    let one = [(AFTER, 640_000_000)];
    let got = parse_plan(&plan(0, 0, &one), DISK).unwrap();
    assert!(got.is_live());
    assert_eq!(got.imports(), &one);
    assert!(!parse_plan(&plan(DATA_FLOOR, VOL, &[]), DISK).unwrap().is_live());
    let p = |b, s, i: &[(u64, u64)]| parse_plan(&plan(b, s, i), DISK).map(|_| ());
    assert_eq!(p(0, 0, &[(256, 512)]), Err(E::BelowFloor));
    assert_eq!(p(0, 0, &[(DISK - 1, 1024)]), Err(E::PastEnd));
    assert_eq!(p(0, 0, &[(AFTER, 1024), (AFTER + 1, 512)]), Err(E::Overlap));
    /* Half a live plan is neither: a base with no sectors, or sectors at 0. */
    assert_eq!(p(DATA_FLOOR, 0, &[]), Err(E::VolumeTooSmall));
    assert_eq!(p(0, VOL, &[]), Err(E::BelowFloor));
}
