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

//! The whole chain the store write runs: a plan sector, parsed by the
//! kernel's parser, mapped as the plan read maps it, then the keep rule.

use crate::keep_rule::{disk_of, may_write, Disk};
use crate::model::{keeps_volume, plan};
use crate::plan_types::DATA_FLOOR;
use crate::sweep::Rng;

const DISK: u64 = 4 * 1024 * 1024;
const VOL: u64 = 1 << 20;
const RECORD: [u8; 512] = [0xA5; 512];

fn disk(sector: Option<&[u8; 512]>) -> Disk {
    disk_of(keeps_volume(sector, DISK))
}

#[test]
fn only_a_plan_with_a_volume_lets_a_record_through() {
    let installed = plan(DATA_FLOOR, VOL, &[]);
    assert_eq!(disk(Some(&installed)), Disk::Installed);
    assert!(may_write(disk(Some(&installed)), &RECORD));
    let live = plan(0, 0, &[(DATA_FLOOR, 4096)]);
    let blank = [0u8; 512];
    for s in [&live, &blank] {
        assert_eq!(disk(Some(s)), Disk::Live);
        assert!(!may_write(disk(Some(s)), &RECORD));
        assert!(may_write(disk(Some(s)), &[0u8; 512]));
    }
}

/* A plan the parser refuses, or no sector at all, never opens the store. */
#[test]
fn a_refused_or_unread_plan_keeps_nothing() {
    let refused = [
        plan(256, VOL, &[]),
        plan(DATA_FLOOR, 100, &[]),
        plan(DATA_FLOOR, DISK, &[]),
        plan(u64::MAX - 5, VOL, &[]),
        plan(DATA_FLOOR, VOL, &[(DATA_FLOOR + 10, 512)]),
    ];
    for s in &refused {
        assert_eq!(disk(Some(s)), Disk::Unread);
        assert!(!may_write(disk(Some(s)), &RECORD));
    }
    assert_eq!(disk(None), Disk::Unread);
}

/* Random sectors, half with the magic: a record passes exactly when the
 * parser accepted a plan that keeps a volume. */
#[test]
fn random_sectors_open_the_store_only_through_a_good_volume_plan() {
    let mut r = Rng::new(0x5eed_0003);
    let mut opened = 0;
    for i in 0..200_000u64 {
        let mut s = [0u8; 512];
        r.fill(&mut s);
        if i % 2 == 0 {
            s[..8].copy_from_slice(b"NONOSDP1");
            let base = DATA_FLOOR + r.below(DISK);
            s[8..16].copy_from_slice(&base.to_le_bytes());
            s[16..24].copy_from_slice(&r.below(DISK).to_le_bytes());
            s[24..32].copy_from_slice(&r.below(4).to_le_bytes());
        }
        let parsed = crate::plan::parse_plan(&s, DISK);
        let good = parsed.is_ok_and(|p| !p.is_live());
        assert_eq!(may_write(disk(Some(&s)), &RECORD), good, "sector {i}");
        opened += good as u32;
    }
    assert!(opened > 1_000, "the sweep reached too few good plans: {opened}");
}
