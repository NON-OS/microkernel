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

//! The keep rule alone: what each kind of disk lets a store write change.

use crate::keep_rule::{disk_of, may_write, Disk};
use crate::sweep::Rng;

const SECTOR: usize = 512;
const MAX: usize = 8192;

#[test]
fn an_installed_disk_takes_any_write() {
    let mut r = Rng::new(0x5eed_0001);
    for _ in 0..20_000 {
        let mut b = vec![0u8; SECTOR * (1 + r.below((MAX / SECTOR) as u64) as usize)];
        r.fill(&mut b);
        assert!(may_write(Disk::Installed, &b));
    }
    assert!(may_write(Disk::Installed, &[0u8; MAX]));
}

#[test]
fn a_disk_that_keeps_nothing_takes_zeros_and_nothing_else() {
    for disk in [Disk::Live, Disk::Unread] {
        for n in (SECTOR..=MAX).step_by(SECTOR) {
            assert!(may_write(disk, &vec![0u8; n]), "{disk:?} refused {n} zeros");
        }
        let mut r = Rng::new(0x5eed_0002);
        for _ in 0..20_000 {
            let mut b = vec![0u8; SECTOR * (1 + r.below((MAX / SECTOR) as u64) as usize)];
            r.fill(&mut b);
            let zeros = b.iter().all(|x| *x == 0);
            assert_eq!(may_write(disk, &b), zeros, "{disk:?}");
        }
    }
}

/* One set bit anywhere is a record: the first byte, the last, every one. */
#[test]
fn one_set_bit_at_any_place_is_refused() {
    for disk in [Disk::Live, Disk::Unread] {
        let mut b = vec![0u8; MAX];
        for at in 0..MAX {
            for bit in 0..8 {
                b[at] = 1 << bit;
                assert!(!may_write(disk, &b), "{disk:?} let byte {at} bit {bit} through");
            }
            b[at] = 0;
        }
    }
}

#[test]
fn each_plan_reading_names_its_disk() {
    assert_eq!(disk_of(Some(true)), Disk::Installed);
    assert_eq!(disk_of(Some(false)), Disk::Live);
    assert_eq!(disk_of(None), Disk::Unread);
}
