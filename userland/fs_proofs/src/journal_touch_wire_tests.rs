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

// These proofs drive the production OP_JOURNAL_TOUCH handler, the one the
// capsule dispatches to, so the path a client sends is the path Recents
// records, and a malformed request leaves the journal as it was.

use alloc::vec::Vec;

use crate::protocol::{Request, EACCES, EINVAL, HDR_LEN, OP_JOURNAL_TOUCH};
use crate::store::Store;
use crate::vfs_handlers::journal::journal_touch;

const PID: u32 = 7;

fn touch(store: &mut Store, claimed_pid: u32, rest: &[u8]) -> i32 {
    let mut payload = Vec::new();
    payload.extend_from_slice(&claimed_pid.to_le_bytes());
    payload.extend_from_slice(rest);
    let req = Request { op: OP_JOURNAL_TOUCH, flags: 0, request_id: 1, payload: &payload };
    let rx = journal_touch(store, req, PID);
    assert_eq!(rx.len(), HDR_LEN + 4, "a touch reply carries a status and no body");
    let mut status = [0u8; 4];
    status.copy_from_slice(&rx[HDR_LEN..HDR_LEN + 4]);
    i32::from_le_bytes(status)
}

fn named(path: &[u8]) -> Vec<u8> {
    let mut rest = Vec::new();
    rest.push(path.len() as u8);
    rest.extend_from_slice(path);
    rest
}

fn recents(store: &Store) -> Vec<&str> {
    store.journal_list(200).into_iter().map(|(_, p)| p).collect()
}

#[test]
fn a_touch_records_the_normalized_path() {
    let mut s = Store::new();
    assert_eq!(touch(&mut s, PID, &named(b"docs//a.txt")), 0);
    assert_eq!(recents(&s), ["/docs/a.txt"]);
}

#[test]
fn a_malformed_touch_is_refused_and_records_nothing() {
    let mut s = Store::new();
    assert_eq!(touch(&mut s, PID, &[]), EINVAL, "no length byte");
    assert_eq!(touch(&mut s, PID, &[0]), EINVAL, "an empty name");
    assert_eq!(touch(&mut s, PID, &[5, b'/', b'a']), EINVAL, "a name shorter than its length");
    assert_eq!(touch(&mut s, PID, &named(&[0xff, 0xfe])), EINVAL, "a name that is not UTF-8");
    assert_eq!(touch(&mut s, PID, &named(b"/a/../b")), EINVAL, "a parent climb");
    assert!(recents(&s).is_empty());
}

#[test]
fn a_touch_claiming_another_caller_is_refused() {
    let mut s = Store::new();
    assert_eq!(touch(&mut s, PID + 1, &named(b"/a")), EACCES);
    assert!(recents(&s).is_empty());
}
