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

use crate::{bucket, PolicyError, SyscallSet, BUCKETS, SET_LEN};

#[test]
fn buckets_follow_the_seal() {
    assert_eq!(bucket(0), 0);
    assert_eq!(bucket(511), 511);
    assert_eq!(bucket(512), 512);
    assert_eq!(bucket(u64::MAX), 512);
    assert_eq!(BUCKETS, 513);
    assert_eq!(SET_LEN * 8 - 7, BUCKETS);
}

#[test]
fn all_allows_every_call_and_none_allows_none() {
    for nr in (0..600).chain([u64::MAX, 1 << 40]) {
        assert!(SyscallSet::ALL.allows(nr), "{nr}");
        assert!(!SyscallSet::NONE.allows(nr), "{nr}");
    }
    assert_eq!(SyscallSet::ALL.as_bytes()[SET_LEN - 1], 1);
}

#[test]
fn with_adds_one_bucket_only() {
    let s = SyscallSet::NONE.with(59);
    for nr in 0..600 {
        assert_eq!(s.allows(nr), nr == 59, "{nr}");
    }
    let s = SyscallSet::NONE.with(700);
    assert!(s.allows(512) && s.allows(u64::MAX) && !s.allows(511));
}

#[test]
fn from_bytes_refuses_bits_past_the_last_bucket() {
    let mut b = [0u8; SET_LEN];
    b[SET_LEN - 1] = 0x02;
    assert_eq!(SyscallSet::from_bytes(b), Err(PolicyError::SyscallSet));
    b[SET_LEN - 1] = 0x01;
    assert!(SyscallSet::from_bytes(b).unwrap().allows(512));
}
