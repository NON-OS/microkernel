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

//! The table counted is the table made, and only growth past it is refused.

use super::super::super::random::Regs;
use super::ceiling::lock;
use crate::linux::abi::errno::ENOLCK;
use crate::linux::file::lock::apply::ranges::{applied, count_after};
use crate::linux::file::lock::room::{fits, MOST_LOCKS};
use crate::linux::file::lock::table::Lock;

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
