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

use super::rights_attach::{attach, Attach};
use super::rights_rule::{grant, revoke_handle, revoke_pid, Grant, Right};

pub(super) const OWNER: u32 = 10;
const RECEIVER: u32 = 20;
/// A thread of RECEIVER's process.
const RECEIVER_THREAD: u32 = 21;
pub(super) const THIRD: u32 = 30;
pub(super) const H: u64 = (3 << 32) | 7;

pub(super) fn same(a: u32, b: u32) -> bool {
    let group = |p: u32| if p == RECEIVER_THREAD { RECEIVER } else { p };
    group(a) == group(b)
}

pub(super) fn shared(write: bool) -> Vec<Right> {
    let mut rights = Vec::new();
    assert_eq!(grant(&mut rights, Right { handle: H, pid: RECEIVER, write }), Grant::New);
    rights
}

#[test]
fn a_third_pid_is_refused() {
    let rights = shared(false);
    assert_eq!(attach(&rights, Some(OWNER), H, THIRD, same), Attach::Refused);
}

#[test]
fn the_receiver_and_its_threads_attach_read_only_and_the_owner_attaches() {
    let rights = shared(false);
    let read_only = Attach::Receiver { write: false };
    assert_eq!(attach(&rights, Some(OWNER), H, RECEIVER, same), read_only);
    assert_eq!(attach(&rights, Some(OWNER), H, RECEIVER_THREAD, same), read_only);
    assert_eq!(attach(&rights, Some(OWNER), H, OWNER, same), Attach::Owner);
    let rights = shared(true);
    let writable = Attach::Receiver { write: true };
    assert_eq!(attach(&rights, Some(OWNER), H, RECEIVER, same), writable);
}

#[test]
fn after_release_or_death_the_receiver_is_refused() {
    let mut rights = shared(false);
    revoke_handle(&mut rights, H);
    assert_eq!(attach(&rights, Some(OWNER), H, RECEIVER, same), Attach::Refused);
    let rights = shared(false);
    assert_eq!(attach(&rights, None, H, RECEIVER, same), Attach::Refused);
    let mut rights = shared(false);
    revoke_pid(&mut rights, RECEIVER);
    assert_eq!(attach(&rights, Some(OWNER), H, RECEIVER, same), Attach::Refused);
}
