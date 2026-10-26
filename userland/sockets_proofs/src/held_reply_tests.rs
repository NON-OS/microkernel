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

//! A connect answered after its turn is answered by the kernel's id for its
//! own call, taken when the request was received, so the answer reaches that
//! call and never a later one from the same caller.

use nonos_libc::ipc::{push_request, IpcReqMeta};
use nonos_libc::{serial, take_replies};

use crate::fresh_table::fresh;
use crate::protocol::{E_NO_TRANSPORT, E_OK, OP_CONNECT};
use crate::server::install_transport::{install_transport, uninstall_transport};
use crate::server::request::{current, receive};
use crate::server::respond::{respond, respond_to};
use crate::server::verdict::kept;
use crate::sockets::{Kind, SOCKETS};

fn le32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

#[test]
fn the_request_taken_last_names_its_id_and_none_after_an_empty_wait() {
    let _guard = serial();
    let meta = IpcReqMeta { version: 1, sender: 44, request: 12, ..Default::default() };
    push_request(b"connect", meta);
    let (mut rx, mut sender) = ([0u8; 32], 0u32);
    assert_eq!((receive(&mut rx, 2, &mut sender), sender, current()), (7, 44, 12));
    assert_eq!((receive(&mut rx, 2, &mut sender), current()), (0, 0));
}

#[test]
fn a_held_connect_is_answered_by_its_id_and_one_in_turn_by_pid() {
    let _guard = serial();
    let _ = take_replies();
    let mut tx = [0u8; 64];
    let _ = respond_to(12, OP_CONNECT, E_NO_TRANSPORT, 5, &mut tx);
    let _ = respond(44, OP_CONNECT, E_NO_TRANSPORT, 6, 0, &mut tx);
    let r = take_replies();
    assert_eq!(r.len(), 2);
    assert_eq!((r[0].pid, r[0].request, r[0].bytes.len()), (0, 12, 20), "by the id");
    assert_eq!((r[1].pid, r[1].request), (44, 0), "by the pid");
    for (reply, rid) in r.iter().zip([5, 6]) {
        assert_eq!(u16::from_le_bytes([reply.bytes[8], reply.bytes[9]]), E_NO_TRANSPORT);
        assert_eq!((le32(&reply.bytes, 12), le32(&reply.bytes, 16)), (rid, 0));
    }
}

#[test]
fn a_connect_whose_caller_gave_up_leaves_the_socket_unconnected() {
    let _g = fresh();
    assert!(kept(true, true));
    assert!(!kept(true, false) && !kept(false, true) && !kept(false, false));
    let key = SOCKETS.open(51, Kind::Stream).expect("a socket");
    assert_eq!(install_transport(key, Kind::Stream, [10, 0, 2, 2], 443, 9), E_OK);
    uninstall_transport(key);
    let left = SOCKETS.with(key, |s| (s.remote.is_none(), s.transport_handle));
    assert_eq!(left, Some((true, 0)), "as the caller believes it is");
}
