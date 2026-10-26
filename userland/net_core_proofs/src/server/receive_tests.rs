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

//! What a lookup held past its turn answers by is the kernel's id for the
//! request it came in: the one received last, and nothing once a wait ends
//! with no request, so a stale id is never kept for a later handler.

use nonos_libc::ipc::{push_request, IpcReqMeta};
use nonos_libc::serial;

use super::receive::{receive, request};

fn meta(sender: u32, request: u64) -> IpcReqMeta {
    IpcReqMeta { version: 1, sender, request, ..Default::default() }
}

#[test]
fn the_request_received_last_names_its_id_and_sender() {
    let _guard = serial();
    push_request(b"one", meta(31, 7));
    push_request(b"two!", meta(32, 9));
    let (mut rx, mut pid) = ([0u8; 64], 0u32);
    assert_eq!((receive(&mut rx, &mut pid), pid, request()), (3, 31, 7));
    assert_eq!((receive(&mut rx, &mut pid), pid, request()), (4, 32, 9));
    assert_eq!(&rx[..4], b"two!");
}

#[test]
fn a_wait_with_no_request_names_none() {
    let _guard = serial();
    push_request(b"one", meta(31, 7));
    let (mut rx, mut pid) = ([0u8; 64], 0u32);
    assert_eq!((receive(&mut rx, &mut pid), request()), (3, 7));
    assert_eq!((receive(&mut rx, &mut pid), request()), (0, 0));
}
