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

//! A call the kernel turned away itself, and a count that saturates.

use super::fixture::{family, SUP};
use crate::foreign::seal::hist::slot;
use crate::foreign::seal::trace::{Row, REFUSED};

fn refused(nr: u64, ret: u64) -> Row {
    Row { seq: 0, pid: 7, what: REFUSED, nr, ret }
}

#[test]
fn a_kernel_refusal_lands_in_bucket_512() {
    let mut f = family(SUP);
    f.absorb(&refused(u64::MAX - 1, -38i64 as u64)); // past what a supervisor is handed
    f.absorb(&refused(0x5845_464D, -1i64 as u64)); // a native number, refused
    assert_eq!(f.hist[slot(512, true)], 2);
}

#[test]
fn a_native_call_that_went_through_counts_as_a_success() {
    let mut f = family(SUP);
    f.absorb(&refused(0x5954_544D, 0));
    assert_eq!((f.hist[slot(512, false)], f.hist[slot(512, true)]), (1, 0));
}

#[test]
fn a_saturated_count_stays_and_is_flagged() {
    let mut f = family(SUP);
    let s = slot(512, true);
    f.hist[s] = u32::MAX - 1;
    f.absorb(&refused(600, -38i64 as u64));
    assert!(!f.saturated, "u32::MAX itself is still a count");
    f.absorb(&refused(600, -38i64 as u64));
    assert!(f.saturated && f.hist[s] == u32::MAX);
}
