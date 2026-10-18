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

//! statfs's count, and copies held open together.

use super::ceiling::kept;
use crate::calls::loadavg::declared::{PRIVATE, PRIVATE_NAMES};
use crate::calls::loadavg::space::quota::{copies_allow, names_free};
use crate::linux::abi::errno::ENOSPC;

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
