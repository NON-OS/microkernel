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
use super::rights_rule::{grant, share_write, Grant, Right, MAX_RECEIVERS, SHARE_WRITE};
use super::tests::{same, shared, H, OWNER, THIRD};

#[test]
fn enumeration_gains_nothing() {
    let rights = shared(false);
    for slot in 0..256u64 {
        for epoch in 0..4u64 {
            let handle = (slot << 32) | epoch;
            let live = (handle == H || slot % 5 == 0).then_some(OWNER);
            assert_eq!(attach(&rights, live, handle, THIRD, same), Attach::Refused);
        }
    }
}

#[test]
fn receivers_are_bounded_and_naming_one_again_takes_no_room() {
    let mut rights = Vec::new();
    for pid in 0..MAX_RECEIVERS as u32 {
        assert_eq!(grant(&mut rights, Right { handle: H, pid, write: false }), Grant::New);
    }
    let again = Right { handle: H, pid: 0, write: true };
    assert_eq!(grant(&mut rights, again), Grant::Already);
    assert!(rights.iter().any(|r| r.pid == 0 && r.write));
    let one_more = Right { handle: H, pid: 99, write: false };
    assert_eq!(grant(&mut rights, one_more), Grant::Full);
    let elsewhere = Right { handle: H + 1, pid: 99, write: false };
    assert_eq!(grant(&mut rights, elsewhere), Grant::New);
}

#[test]
fn share_flags_name_write_and_nothing_else() {
    assert_eq!(share_write(0), Some(false));
    assert_eq!(share_write(SHARE_WRITE), Some(true));
    assert_eq!(share_write(2), None);
    assert_eq!(share_write(u64::MAX), None);
}
