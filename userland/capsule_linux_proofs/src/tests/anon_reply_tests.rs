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

//! What the capsule says to net.anon's handle front and how it reads the
//! answers. The values are held to net.anon's own files, the header to the
//! lines that write and read it there, and every reply, however short,
//! long, garbled or unknown, ends in a clean errno and never a panic.

use super::mutation::damage;
use super::random::Regs;
use crate::anon_server_end::{REASON_DONE as SERVER_DONE, REASON_EXITPOLICY};
use crate::anon_server_errno as server;
use crate::anon_server_ops as server_ops;
use crate::linux::abi::errno::{EAGAIN, ECONNREFUSED, EIO, ENETUNREACH, ENOBUFS, EPIPE};
use crate::linux::net::anon_answer::{dotted, got, open_body, opened, sent};
use crate::linux::net::anon_ops::*;
use crate::linux::net::anon_wire::{arrived, reply, request, REQUEST_ID};
use crate::linux::net::sock::Got;

const LIMITS: &str = include_str!("../../../capsule_net_anon/src/protocol/limits.rs");
const RESPOND: &str = include_str!("../../../capsule_net_anon/src/server/respond.rs");
const PARSE: &str = include_str!("../../../capsule_net_anon/src/server/parse_req.rs");
const TABLE: &str = include_str!("../../../capsule_net_anon/src/stream/table.rs");

/// A reply as net.anon's respond.rs writes it.
fn server_reply(op: u16, status: u16, request_id: u32, body: &[u8]) -> Vec<u8> {
    let mut tx = Vec::new();
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&VERSION.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&status.to_le_bytes());
    tx.extend_from_slice(&[0, 0]);
    tx.extend_from_slice(&request_id.to_le_bytes());
    tx.extend_from_slice(&(body.len() as u32).to_le_bytes());
    tx.extend_from_slice(body);
    tx
}

#[test]
fn every_value_is_net_anons_own() {
    assert_eq!(
        [OP_OPEN_STREAM, OP_SEND, OP_RECV, OP_CLOSE_STREAM],
        [
            server_ops::OP_OPEN_STREAM,
            server_ops::OP_SEND,
            server_ops::OP_RECV,
            server_ops::OP_CLOSE_STREAM
        ]
    );
    assert_eq!(
        [E_OK, E_BAD_LEN, E_NO_TCP, E_NO_DIRECTORY, E_NO_PATH, E_NO_LINK, E_TABLE_FULL],
        [
            server::E_OK,
            server::E_BAD_LEN,
            server::E_NO_TCP,
            server::E_NO_DIRECTORY,
            server::E_NO_PATH,
            server::E_NO_LINK,
            server::E_TABLE_FULL
        ]
    );
    assert_eq!(
        [E_NO_CIRCUIT, E_NO_STREAM, E_RX_EMPTY, E_STREAM_CLOSED, E_DIRECTORY_STALE, E_WOULD_BLOCK],
        [
            server::E_NO_CIRCUIT,
            server::E_NO_STREAM,
            server::E_RX_EMPTY,
            server::E_STREAM_CLOSED,
            server::E_DIRECTORY_STALE,
            server::E_WOULD_BLOCK
        ]
    );
    assert_eq!(REASON_DONE, SERVER_DONE);
    assert!(TABLE.contains(&format!("pub const REASON_DESTROY: u8 = {REASON_DESTROY};")));
    assert!(LIMITS.contains("pub const MAGIC: u32 = 0x414E_4F31;"));
    assert_eq!(MAGIC, 0x414E_4F31);
    assert!(LIMITS.contains(&format!("pub const VERSION: u16 = {VERSION};")));
    assert!(LIMITS.contains(&format!("pub const HDR_LEN: usize = {HDR_LEN};")));
    assert!(LIMITS.contains("pub const IPC_PAYLOAD_MAX: usize = 32 * 1024;"));
    assert_eq!(PAYLOAD_MAX, 32 * 1024);
}

#[test]
fn the_header_is_laid_out_where_net_anon_reads_and_writes_it() {
    for line in [
        "if le32(buf, 0) != MAGIC",
        "if le16(buf, 4) != VERSION",
        "let payload_len = le32(buf, 16) as usize;",
        "Request { op: le16(buf, 6), request_id: le32(buf, 12) }",
    ] {
        assert!(PARSE.contains(line), "parse_req.rs no longer reads {line}");
    }
    for line in [
        "tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());",
        "tx[4..6].copy_from_slice(&VERSION.to_le_bytes());",
        "tx[6..8].copy_from_slice(&op.to_le_bytes());",
        "tx[8..10].copy_from_slice(&errno.to_le_bytes());",
        "tx[12..16].copy_from_slice(&request_id.to_le_bytes());",
        "tx[16..20].copy_from_slice(&payload_len.to_le_bytes());",
    ] {
        assert!(RESPOND.contains(line), "respond.rs no longer writes {line}");
    }
    let tx = request(OP_SEND, b"\x07\x00hello");
    assert_eq!(tx.len(), HDR_LEN + 7);
    assert_eq!(&tx[0..4], &MAGIC.to_le_bytes());
    assert_eq!(&tx[4..6], &VERSION.to_le_bytes());
    assert_eq!(&tx[6..8], &OP_SEND.to_le_bytes());
    assert_eq!(&tx[12..16], &REQUEST_ID.to_le_bytes());
    assert_eq!(&tx[16..20], &7u32.to_le_bytes());
    assert_eq!(&tx[HDR_LEN..], b"\x07\x00hello");
    /* Never one of the SOCKS front's markers, which net.anon tells apart by the first byte. */
    assert!(!matches!(tx[0], 0..=2));
}

#[test]
fn a_reply_is_read_only_when_it_is_the_reply_asked_for() {
    let good = server_reply(OP_RECV, E_OK, REQUEST_ID, b"abc");
    assert_eq!(reply(&good, OP_RECV), Some((E_OK, &b"abc"[..])));
    /* Too short to hold a header, at every length. */
    for n in 0..HDR_LEN {
        assert_eq!(reply(&good[..n], OP_RECV), None, "{n} bytes");
    }
    /* Cut inside the body, or with bytes past it: the length is not what came. */
    assert_eq!(reply(&good[..good.len() - 1], OP_RECV), None);
    let mut long = good.clone();
    long.push(0);
    assert_eq!(reply(&long, OP_RECV), None);
    /* Another op, another request, another magic or version. */
    assert_eq!(reply(&good, OP_SEND), None);
    assert_eq!(reply(&server_reply(OP_RECV, E_OK, REQUEST_ID + 1, b"abc"), OP_RECV), None);
    for at in [0, 4] {
        let mut bad = good.clone();
        bad[at] ^= 1;
        assert_eq!(reply(&bad, OP_RECV), None, "byte {at}");
    }
    /* A length past anything net.anon sends, whatever arrived. */
    let mut huge = server_reply(OP_RECV, E_OK, REQUEST_ID, &[]);
    huge[16..20].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(reply(&huge, OP_RECV), None);
    assert_eq!(
        reply(&server_reply(OP_RECV, E_OK, REQUEST_ID, &vec![1; PAYLOAD_MAX + 1]), OP_RECV),
        None
    );
    /* What a call returned is believed only inside the buffer. */
    assert_eq!(arrived(-1, 64), None);
    assert_eq!(arrived(65, 64), None);
    assert_eq!(arrived(64, 64), Some(64));
}

#[test]
fn an_open_is_unreachable_without_transport_and_refused_otherwise() {
    assert_eq!(opened(Some((E_OK, &[0x34, 0x12]))), Ok(0x1234));
    for status in [E_NO_TCP, E_NO_DIRECTORY, E_DIRECTORY_STALE, E_NO_PATH, E_NO_LINK, E_NO_CIRCUIT]
    {
        assert_eq!(opened(Some((status, &[]))), Err(ENETUNREACH), "status {status}");
    }
    assert_eq!(opened(Some((E_TABLE_FULL, &[]))), Err(ENOBUFS), "this capsule's share is used");
    assert_eq!(opened(Some((E_BAD_LEN, &[]))), Err(ECONNREFUSED));
    assert_eq!(opened(None), Err(EIO), "no answer");
    /* An id that is short or long, and a status nobody sends. */
    assert_eq!(opened(Some((E_OK, &[]))), Err(EIO));
    assert_eq!(opened(Some((E_OK, &[1]))), Err(EIO));
    assert_eq!(opened(Some((E_OK, &[1, 2, 3]))), Err(EIO));
    for status in [2, 3, 12, 13, 16, 17, 19, 21, u16::MAX] {
        assert_eq!(opened(Some((status, &[1, 0]))), Err(EIO), "status {status}");
    }
}

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

#[test]
fn hostile_replies_end_in_an_errno_and_never_a_panic() {
    let mut r = Regs::new(0x0a40_41e5);
    let mut seed = 0x0dd5_eed0_u64;
    let statuses = [
        E_OK,
        E_NO_LINK,
        E_TABLE_FULL,
        E_NO_STREAM,
        E_RX_EMPTY,
        E_STREAM_CLOSED,
        E_WOULD_BLOCK,
        77,
    ];
    for round in 0..20_000 {
        let op = [OP_OPEN_STREAM, OP_SEND, OP_RECV][round % 3];
        let status = statuses[r.small(statuses.len() as u64) as usize];
        let body: Vec<u8> = (0..r.small(9)).map(|_| r.any() as u8).collect();
        let mut raw = server_reply(op, status, REQUEST_ID, &body);
        if r.small(2) == 0 {
            damage(&mut seed, &mut raw);
        }
        let read = reply(&raw, op);
        if let Some((_, b)) = read {
            assert!(b.len() <= PAYLOAD_MAX);
        }
        match opened(read) {
            Ok(_) => assert!(matches!(read, Some((E_OK, b)) if b.len() == 2)),
            Err(e) => assert!([ENETUNREACH, ECONNREFUSED, ENOBUFS, EIO].contains(&e), "open: {e}"),
        }
        let asked = r.small(64) as usize;
        match sent(read, asked) {
            Ok(n) => assert!(n <= asked),
            Err(e) => assert!([EAGAIN, EPIPE, EIO].contains(&e), "send: {e}"),
        }
        let _ = got(read);
    }
}

#[test]
fn a_host_goes_as_written_and_an_address_as_its_dotted_quad() {
    assert_eq!(open_body(b"example.com", 443), Some(b"\xbb\x01example.com".to_vec()));
    assert_eq!(dotted([93, 184, 216, 34]), b"93.184.216.34".to_vec());
    assert_eq!(dotted([0, 0, 0, 0]), b"0.0.0.0".to_vec());
    assert_eq!(
        open_body(&dotted([255, 255, 255, 255]), 1),
        Some(b"\x01\x00255.255.255.255".to_vec())
    );
    /* What a BEGIN's "host:port" cannot carry as written. */
    assert_eq!(open_body(b"", 80), None);
    assert_eq!(open_body(&[b'a'; 254], 80), None);
    assert!(open_body(&[b'a'; 253], 80).is_some());
    assert_eq!(open_body(b"evil.com:25\0", 80), None);
    assert_eq!(open_body(b"a\0b", 80), None);
    assert_eq!(open_body(b"a:b", 80), None);
}
