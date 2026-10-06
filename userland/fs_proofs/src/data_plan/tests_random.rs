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

//! Random plans, and where the plan's own sector and the key header sit.

use super::key_header::KEY_LBA;
use super::plan::parse_plan;
use super::plan_types::{PlanError, DATA_FLOOR, MAX_IMPORTS, PLAN_LBA};
use super::tests::{plan, AFTER, DISK, VOL};

#[test]
fn the_plan_sector_lies_past_the_store_and_below_everything_it_names() {
    /*
     * The store starts at LBA 256 and holds at most 16 MiB: 32,768 sectors.
     */
    const { assert!(256 + 32_768 <= PLAN_LBA) };
    const { assert!(PLAN_LBA < DATA_FLOOR) };
}

#[test]
fn the_key_header_lies_after_the_plan_where_no_plan_range_may_reach() {
    const { assert!(PLAN_LBA < KEY_LBA && KEY_LBA < DATA_FLOOR) };
    let below = Err(PlanError::BelowFloor);
    assert_eq!(parse_plan(&plan(KEY_LBA, VOL, &[]), DISK), below);
    assert_eq!(parse_plan(&plan(DATA_FLOOR, VOL, &[(KEY_LBA, 512)]), DISK), below);
}

#[test]
fn random_sectors_with_the_magic_never_panic_the_parser() {
    let mut seed = 0x51_7cc1_b727_220a_u64;
    for _ in 0..100_000 {
        let mut s = [0u8; 512];
        s[..8].copy_from_slice(b"NONOSDP1");
        for b in s[8..].iter_mut() {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            *b = seed as u8;
        }
        /*
         * A count the sector can hold, so the entries are read, not refused.
         */
        s[24..32].copy_from_slice(&(seed % (MAX_IMPORTS as u64 + 2)).to_le_bytes());
        if let Ok(p) = parse_plan(&s, DISK) {
            assert!(p.volume_base >= DATA_FLOOR && p.volume_base + p.volume_sectors <= DISK);
            assert!(p.imports().iter().all(|&(a, b)| a >= DATA_FLOOR && b > 0));
        }
    }
}

#[test]
fn a_full_sector_of_imports_is_read_to_the_last() {
    let all: [(u64, u64); MAX_IMPORTS] = core::array::from_fn(|i| (AFTER + i as u64 * 8, 4096));
    let got = parse_plan(&plan(DATA_FLOOR, VOL, &all), DISK).unwrap();
    assert_eq!(got.imports(), &all);
}
