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

//! What a family may keep in its private directories. They live in the
//! store every capsule shares, which holds 2048 names in all and keeps its
//! bytes in one 192 MiB heap, so the quota is what keeps one guest from
//! filling the store or running it out of memory. Held here: the ceilings
//! are ones the store can carry, a family is refused past either with
//! ENOSPC however it gets there, a family over the quota can still shrink,
//! and the copies it writes into, held in this capsule's memory until they
//! reach the store, stop at the same ceiling.

use super::random::Regs;
use crate::calls::loadavg::declared::{MEMORY, PRIVATE, PRIVATE_NAMES};
use crate::calls::loadavg::space::quota::{allows, copies_allow, names_free, Kept};
use crate::linux::abi::errno::ENOSPC;

/// capsule_vfs: the names the whole store holds, and the heap its bytes
/// live in beside the 60 MiB it loads at boot.
const STORE_NAMES: u64 = 2048;
const STORE_HEAP: u64 = 192 << 20;
const STORE_LOADED: u64 = 60 << 20;

fn kept(bytes: u64, names: u64) -> Kept {
    Kept { bytes, names }
}

/// Checked as the crate compiles: a ceiling raised past what the store
/// carries fails the build of these proofs.
#[test]
fn the_quota_is_one_the_store_can_carry_for_several_families() {
    const { assert!(PRIVATE < MEMORY / 2, "not the half of memory an unsized tmpfs takes") };
    const { assert!(8 * PRIVATE <= STORE_HEAP - STORE_LOADED, "eight families fit the heap") };
    const { assert!(8 * PRIVATE_NAMES <= STORE_NAMES / 2, "eight take half the names") };
    const { assert!(PRIVATE >= 8 << 20, "one file as large as a family may hold fits") };
}

#[test]
fn a_family_within_its_quota_is_never_refused() {
    assert_eq!(allows(kept(0, 0), kept(PRIVATE, PRIVATE_NAMES)), Ok(()));
    assert_eq!(allows(kept(PRIVATE - 1, 0), kept(PRIVATE, 1)), Ok(()));
}

#[test]
fn a_change_past_either_ceiling_is_enospc() {
    assert_eq!(allows(kept(PRIVATE, 0), kept(PRIVATE + 1, 0)), Err(ENOSPC));
    assert_eq!(allows(kept(0, PRIVATE_NAMES), kept(0, PRIVATE_NAMES + 1)), Err(ENOSPC));
    assert_eq!(allows(kept(0, 0), kept(u64::MAX, 0)), Err(ENOSPC));
    let full = kept(PRIVATE, PRIVATE_NAMES);
    assert_eq!(allows(full, full.plus(kept(0, 1))), Err(ENOSPC), "one more name");
    assert_eq!(allows(full, full.plus(kept(1, 0))), Err(ENOSPC), "one more byte");
}

/// The guest that made the bug: one file after another in /tmp, or one
/// file grown a megabyte at a time. Each step asks the rule, and the
/// family stops at the ceiling instead of at the store's.
#[test]
fn making_names_or_bytes_forever_stops_at_the_ceiling() {
    let mut now = kept(6, 6);
    let mut made = 0u64;
    while allows(now, now.plus(kept(0, 1))).is_ok() {
        now = now.plus(kept(0, 1));
        made += 1;
        assert!(made <= STORE_NAMES, "never refused");
    }
    assert_eq!(now.names, PRIVATE_NAMES);
    let mut now = kept(0, 1);
    while allows(now, now.plus(kept(1 << 20, 0))).is_ok() {
        now = now.plus(kept(1 << 20, 0));
    }
    assert_eq!(now.bytes, PRIVATE);
}

/// A family over its quota, as one that sent a write past it before the
/// copy reached the store is, can still give space back.
#[test]
fn a_family_over_its_quota_can_still_shrink() {
    let over = kept(PRIVATE + 5, PRIVATE_NAMES + 2);
    assert_eq!(allows(over, kept(PRIVATE + 4, PRIVATE_NAMES + 2)), Ok(()));
    assert_eq!(allows(over, kept(0, 0)), Ok(()));
    assert_eq!(allows(over, over), Ok(()), "a write that grows nothing");
    assert_eq!(allows(over, kept(PRIVATE + 6, PRIVATE_NAMES)), Err(ENOSPC));
}

#[test]
fn every_change_that_ends_past_a_ceiling_and_grew_is_refused() {
    let mut r = Regs::new(0x0907_A5ED);
    for _ in 0..200_000 {
        let now = kept(r.small(2 * PRIVATE), r.small(2 * PRIVATE_NAMES));
        let then = kept(r.small(2 * PRIVATE), r.small(2 * PRIVATE_NAMES));
        let grew_past = (then.bytes > PRIVATE && then.bytes > now.bytes)
            || (then.names > PRIVATE_NAMES && then.names > now.names);
        assert_eq!(allows(now, then).is_err(), grew_past, "{now:?} to {then:?}");
    }
}

#[test]
fn statfs_counts_the_names_left() {
    assert_eq!(names_free(kept(0, 0)), PRIVATE_NAMES);
    assert_eq!(names_free(kept(0, 6)), PRIVATE_NAMES - 6);
    assert_eq!(names_free(kept(0, PRIVATE_NAMES + 3)), 0, "over the quota is none left");
}

/// The copies a family writes into are this capsule's memory. A guest that
/// opens file after file and grows each to the most one may hold, closing
/// none, has nothing in the store yet; the copies stop at the quota all
/// the same, far inside the capsule's heap.
#[test]
fn copies_held_open_together_stop_at_the_quota() {
    const MAX_FILE: u64 = 8 << 20;
    let mut files = vec![0u64; 64];
    let mut me = 0;
    'open: while me < files.len() {
        while files[me] < MAX_FILE {
            let total: u64 = files.iter().sum();
            let want = files[me] + (1 << 20);
            if copies_allow(total, files[me], want).is_err() {
                break 'open;
            }
            files[me] = want;
        }
        me += 1;
    }
    let total: u64 = files.iter().sum();
    assert_eq!(total, PRIVATE, "the copies stop at the quota, not at the heap");
    assert_eq!(copies_allow(total, files[0], files[0]), Ok(()), "a rewrite in place");
    assert_eq!(copies_allow(total, files[0], 0), Ok(()), "a truncate gives back");
    assert_eq!(copies_allow(total, files[0], files[0] + 1), Err(ENOSPC));
    assert_eq!(copies_allow(0, 0, u64::MAX), Err(ENOSPC));
}
