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

//! Growing forever stops at the ceiling; shrinking is always taken.

use super::super::super::random::Regs;
use super::ceiling::{kept, STORE_NAMES};
use crate::calls::loadavg::declared::{PRIVATE, PRIVATE_NAMES};
use crate::calls::loadavg::space::quota::allows;
use crate::linux::abi::errno::ENOSPC;

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
