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

//! A session read: an empty one ends the session only when the connection's
//! POLL says the client finished or the connection is gone, so a client that
//! is only quiet keeps its session and one that left frees nsh.front at once.

use core::cell::Cell;

use crate::accepted::E_NO_TRANSPORT;
use crate::read::{read_is, Read, POLL_FINISHED};

const READABLE: u8 = 0x01;
const WRITABLE: u8 = 0x02;
const SOCK: &str = include_str!("../../../capsule_nsh_front/src/sock.rs");

fn unasked() -> Option<(u16, Option<u8>)> {
    panic!("POLL asked for a read that answered")
}

#[test]
fn bytes_are_data_and_poll_is_not_asked() {
    assert_eq!(read_is(Some((0, 1)), unasked), Read::Data);
    assert_eq!(read_is(Some((0, 32 * 1024)), unasked), Read::Data);
    assert_eq!(read_is(Some((0, 0)), unasked), Read::Idle);
}

#[test]
fn a_quiet_client_keeps_its_session() {
    for bits in [0, READABLE, WRITABLE, READABLE | WRITABLE] {
        assert_eq!(read_is(Some((E_NO_TRANSPORT, 0)), || Some((0, Some(bits)))), Read::Idle);
    }
}

#[test]
fn a_finished_client_ends_it() {
    for bits in [POLL_FINISHED, POLL_FINISHED | WRITABLE, 0xFF] {
        let read = read_is(Some((E_NO_TRANSPORT, 0)), || Some((0, Some(bits))));
        assert_eq!(read, Read::Gone, "bits {bits:#x}");
    }
}

#[test]
fn a_poll_that_fails_or_says_nothing_ends_it() {
    assert_eq!(read_is(Some((E_NO_TRANSPORT, 0)), || None), Read::Gone);
    assert_eq!(read_is(Some((E_NO_TRANSPORT, 0)), || Some((0, None))), Read::Gone);
    for errno in 1..=u16::MAX {
        let read = read_is(Some((E_NO_TRANSPORT, 0)), || Some((errno, Some(WRITABLE))));
        assert_eq!(read, Read::Gone, "errno {errno}");
    }
}

#[test]
fn every_other_read_answer_ends_it_without_a_poll() {
    assert_eq!(read_is(None, unasked), Read::Gone);
    for errno in (1..=u16::MAX).filter(|e| *e != E_NO_TRANSPORT) {
        assert_eq!(read_is(Some((errno, 0)), unasked), Read::Gone, "errno {errno}");
    }
}

#[test]
fn poll_is_asked_once_and_only_for_an_empty_read() {
    let asked = Cell::new(0);
    let _ = read_is(Some((E_NO_TRANSPORT, 0)), || {
        asked.set(asked.get() + 1);
        Some((0, Some(WRITABLE)))
    });
    assert_eq!(asked.get(), 1);
}

#[test]
fn the_bit_and_op_are_the_services_own() {
    let core = include_str!("../../../capsule_net_core/src/protocol/tcp.rs");
    assert!(core.contains(&format!("pub const POLL_FINISHED: u8 = 0x{POLL_FINISHED:02x};")));
    let ops = include_str!("../../../capsule_net_sockets/src/protocol/ops.rs");
    assert!(ops.contains("pub const OP_POLL: u16 = 13;"));
    assert!(SOCK.contains("const OP_POLL: u16 = 13;"));
    // net.sockets hands net.core's POLL byte on as it came.
    let status = include_str!("../../../capsule_net_sockets/src/clients/tcp_status.rs");
    assert!(status.contains("Ok(out[0])"));
}

#[test]
fn the_session_loop_reads_through_the_rule() {
    assert!(SOCK
        .contains("read_is(answer.map(|(errno, _, plen)| (errno, plen)), || self.poll(handle))"));
    let session = include_str!("../../../capsule_nsh_front/src/session.rs");
    assert!(session.contains("Recv::Idle => {}"));
    assert!(session.contains("Recv::Gone => break,"));
}
