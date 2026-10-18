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

//! What a send moves and what a read gives.

use crate::anon_server_end::REASON_EXITPOLICY;
use crate::linux::abi::errno::{EAGAIN, EIO, EPIPE};
use crate::linux::net::anon_answer::{got, sent};
use crate::linux::net::anon_ops::*;
use crate::linux::net::sock::Got;

#[test]
fn a_send_moves_what_net_anon_took_and_no_more() {
    let n = |v: u32| v.to_le_bytes();
    assert_eq!(sent(Some((E_OK, &n(10))), 10), Ok(10));
    assert_eq!(sent(Some((E_OK, &n(4))), 10), Ok(4), "a window that closed partway");
    assert_eq!(sent(Some((E_OK, &n(0))), 0), Ok(0));
    assert_eq!(sent(Some((E_OK, &n(0))), 10), Err(EAGAIN), "nothing taken: wait");
    assert_eq!(sent(Some((E_WOULD_BLOCK, &[])), 10), Err(EAGAIN));
    assert_eq!(sent(Some((E_OK, &n(11))), 10), Err(EIO), "more than was sent");
    assert_eq!(sent(Some((E_OK, &n(u32::MAX))), 10), Err(EIO));
    assert_eq!(sent(Some((E_OK, &[1, 0])), 10), Err(EIO), "a short count");
    assert_eq!(sent(Some((E_OK, &[1, 0, 0, 0, 0])), 10), Err(EIO), "a long count");
    assert_eq!(sent(Some((E_WOULD_BLOCK, &[1])), 10), Err(EIO));
    for status in [E_STREAM_CLOSED, E_NO_STREAM, E_NO_LINK, E_NO_CIRCUIT] {
        assert_eq!(sent(Some((status, &[REASON_EXITPOLICY, 1, 0])), 10), Err(EPIPE));
    }
    assert_eq!(sent(None, 10), Err(EIO));
    assert_eq!(sent(Some((999, &[])), 10), Err(EIO));
}

#[test]
fn a_read_from_net_anon_is_bytes_nothing_an_end_or_broken() {
    assert_eq!(got(Some((E_OK, b"hi"))), Got::Bytes(b"hi".to_vec()));
    assert_eq!(got(Some((E_RX_EMPTY, &[]))), Got::Nothing);
    assert_eq!(got(Some((E_STREAM_CLOSED, &[REASON_DONE, 0, 1]))), Got::End(REASON_DONE));
    assert_eq!(got(Some((E_NO_STREAM, &[]))), Got::Gone, "a stream id net.anon does not hold");
    assert_eq!(got(None), Got::Silent);
    /* Empty bytes, bytes with an empty answer, an END of the wrong length, an unknown status. */
    assert_eq!(got(Some((E_OK, &[]))), Got::Garbled);
    assert_eq!(got(Some((E_RX_EMPTY, &[0]))), Got::Garbled);
    assert_eq!(got(Some((E_STREAM_CLOSED, &[]))), Got::Garbled);
    assert_eq!(got(Some((E_STREAM_CLOSED, &[REASON_DONE, 0]))), Got::Garbled);
    assert_eq!(got(Some((E_STREAM_CLOSED, &[REASON_DONE, 0, 1, 0]))), Got::Garbled);
    assert_eq!(got(Some((E_WOULD_BLOCK, &[]))), Got::Garbled);
    assert_eq!(got(Some((u16::MAX, b"hi"))), Got::Garbled);
    assert_eq!(got(Some((E_OK, &vec![0; PAYLOAD_MAX + 1]))), Got::Garbled);
}
