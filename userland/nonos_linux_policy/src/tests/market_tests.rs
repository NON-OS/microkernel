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

//! market_ready: exit and exit_group in the set, bucket 512 out of it, and
//! no FILES_USER in the box.

use crate::market::{ALWAYS_SERVED, NATIVE_BUCKET};
use crate::mask::{FILES_HOME, FILES_USER, WAYLAND};
use crate::{market_ready, unlisted, LinuxAppPolicy, SyscallSet};

fn with(calls: SyscallSet) -> LinuxAppPolicy {
    LinuxAppPolicy { syscall_set: calls, ..unlisted([3; 32]) }
}

#[test]
fn a_set_with_both_exits_and_no_native_bucket_is_ready() {
    assert!(market_ready(&with(SyscallSet::NONE.with(60).with(231))));
    assert!(market_ready(&with(SyscallSet::NONE.with(0).with(60).with(231).with(511))));
}

#[test]
fn either_exit_missing_is_not_ready() {
    assert!(!market_ready(&with(SyscallSet::NONE.with(60))));
    assert!(!market_ready(&with(SyscallSet::NONE.with(231))));
    assert!(!market_ready(&with(SyscallSet::NONE)));
}

#[test]
fn the_native_bucket_is_never_ready_so_all_ones_is_not() {
    let both = SyscallSet::NONE.with(60).with(231);
    assert!(!market_ready(&with(both.with(NATIVE_BUCKET))));
    assert!(!market_ready(&with(both.with(9999))), "past 511 is the native bucket");
    assert!(!market_ready(&with(SyscallSet::ALL)));
    assert!(!market_ready(&unlisted([4; 32])), "the owner's default is all ones");
    assert_eq!(ALWAYS_SERVED, [60, 231]);
}

#[test]
fn files_user_is_reserved_until_something_serves_it() {
    let both = SyscallSet::NONE.with(60).with(231);
    let box_of = |mask| LinuxAppPolicy { mask, ..with(both) };
    assert!(market_ready(&box_of(WAYLAND | FILES_HOME)));
    assert!(!market_ready(&box_of(FILES_USER)));
    assert!(!market_ready(&box_of(WAYLAND | FILES_HOME | FILES_USER)));
}
