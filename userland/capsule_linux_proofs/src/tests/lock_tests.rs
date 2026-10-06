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

//! The family's lock table is this capsule's memory, and a record lock is
//! split and never merged. Held here: the table stops at MOST_LOCKS with
//! ENOLCK however a guest asks, an unlock and a lock over its owner's own
//! are still taken at the ceiling, and the count the ceiling is checked
//! against is the table the change makes.

use super::random::Regs;
use crate::linux::abi::errno::ENOLCK;
use crate::linux::file::lock::apply::{applied, count_after};
use crate::linux::file::lock::room::{fits, MOST_LOCKS};
use crate::linux::file::lock::table::{Lock, Owner};

fn lock(owner: u32, start: u64, end: u64) -> Lock {
    Lock { file: b"/tmp/db".to_vec(), owner: Owner::Posix(owner), write: true, start, end }
}

/// `apply` as the capsule runs it, over a table held here.
fn apply(all: &mut Vec<Lock>, want: &Lock, add: bool) -> Result<(), i64> {
    fits(all.len(), count_after(all, want, add))?;
    *all = applied(core::mem::take(all), want, add);
    Ok(())
}

/// The guest that grew the table: every other byte of a file locked, one
/// call each, twenty thousand times.
#[test]
fn locking_every_other_byte_stops_at_the_ceiling() {
    let mut all = Vec::new();
    let mut refused = 0;
    for i in 0..20_000u64 {
        if apply(&mut all, &lock(7, 2 * i, 2 * i + 1), true) == Err(ENOLCK) {
            refused += 1;
        }
    }
    assert_eq!(all.len(), MOST_LOCKS);
    assert_eq!(refused, 20_000 - MOST_LOCKS);
}

#[test]
fn a_full_table_still_takes_what_does_not_grow_it() {
    let mut all: Vec<Lock> = (0..MOST_LOCKS as u64).map(|i| lock(7, 2 * i, 2 * i + 1)).collect();
    assert_eq!(apply(&mut all, &lock(7, 0, 1), true), Ok(()), "a lock over its owner's own");
    assert_eq!(apply(&mut all, &lock(7, 0, 1), false), Ok(()), "an unlock");
    assert_eq!(all.len(), MOST_LOCKS - 1);
    assert_eq!(apply(&mut all, &lock(8, 1, 2), true), Ok(()), "back to the ceiling");
    assert_eq!(apply(&mut all, &lock(9, 3, 4), true), Err(ENOLCK));
    assert_eq!(apply(&mut all, &lock(7, 2, 5), false), Ok(()), "an unlock over two of them");
}

/// An unlock in the middle of a lock leaves two pieces, which at the
/// ceiling is one record too many.
#[test]
fn a_splitting_unlock_at_the_ceiling_is_enolck() {
    let mut all: Vec<Lock> = (1..MOST_LOCKS as u64).map(|i| lock(i as u32, i, i + 1)).collect();
    all.push(lock(0, 10_000, 20_000));
    assert_eq!(apply(&mut all, &lock(0, 15_000, 15_001), false), Err(ENOLCK));
    assert_eq!(all.len(), MOST_LOCKS, "and nothing changed");
}

/// The ceiling is checked against the table the change makes: over random
/// tables and changes, the count and the table agree, and a change refused
/// leaves the table as it was.
#[test]
fn the_counted_table_is_the_table_made() {
    let mut r = Regs::new(0x0F10_C4ED);
    for _ in 0..20_000 {
        let n = r.small(12) as usize;
        let all: Vec<Lock> = (0..n)
            .map(|_| {
                let s = r.small(64);
                lock(r.small(3) as u32, s, s + 1 + r.small(32))
            })
            .collect();
        let s = r.small(64);
        let want = lock(r.small(3) as u32, s, s + 1 + r.small(32));
        let add = r.small(2) == 0;
        assert_eq!(count_after(&all, &want, add), applied(all.clone(), &want, add).len());
    }
}

#[test]
fn the_ceiling_refuses_only_growth_past_it() {
    assert_eq!(fits(0, MOST_LOCKS), Ok(()));
    assert_eq!(fits(MOST_LOCKS, MOST_LOCKS + 1), Err(ENOLCK));
    assert_eq!(fits(MOST_LOCKS + 5, MOST_LOCKS + 5), Ok(()));
    assert_eq!(fits(MOST_LOCKS + 5, MOST_LOCKS + 6), Err(ENOLCK));
}
