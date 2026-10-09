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

//! A hundred thousand hostile segments against live connections: the state
//! machine never panics, and what it holds stays inside its bounds.

use super::handshake::local_port;
use crate::peer::{connect, encode, fresh, inject, inject_raw, numbers, recv, request, reset};
use crate::peer::{send, Seg, LOCAL, REMOTE};
use crate::protocol::{OP_CLOSE, OP_LISTEN};
use crate::state::RX_DEPTH;
use crate::tcp::MSS;

const LISTEN_PORT: u16 = 8080;

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

/// Near `base` most of the time, anywhere some of the time.
fn near(s: &mut u32, base: u32) -> u32 {
    match xorshift(s) % 8 {
        0 => xorshift(s),
        1 => base.wrapping_sub(xorshift(s) % 70_000),
        _ => base.wrapping_add(xorshift(s) % 70_000),
    }
}

fn hostile(s: &mut u32, conn: u32) -> Seg {
    let (snd_nxt, rcv_nxt, _) = numbers(conn);
    let dport = match xorshift(s) % 4 {
        0 => LISTEN_PORT,
        1 => xorshift(s) as u16,
        _ => local_port(conn),
    };
    let mut seg = Seg::from_peer(dport, near(s, rcv_nxt), near(s, snd_nxt), xorshift(s) as u8 & 0x3F, &[]);
    seg.window = xorshift(s) as u16;
    if xorshift(s).is_multiple_of(4) {
        seg.options = (0..(xorshift(s) % 3) * 4).map(|_| xorshift(s) as u8).collect();
    }
    let len = match xorshift(s) % 4 {
        0 => 0,
        1 => (xorshift(s) % MSS as u32) as usize,
        _ => (xorshift(s) % 64) as usize,
    };
    seg.payload = (0..len).map(|_| xorshift(s) as u8).collect();
    seg
}

fn bounded() {
    let mut t = crate::state::TABLE.lock();
    for e in t.entries_mut().iter() {
        assert!(e.rx.len() <= RX_DEPTH, "the receive queue holds at most {RX_DEPTH} pieces");
        assert!(e.rx.iter().all(|b| b.len() <= MSS), "no queued piece is larger than a segment");
        assert!(e.snd_buf.len() <= crate::tcp::SND_BUF_MAX);
    }
}

#[test]
fn a_hundred_thousand_hostile_segments_never_panic_or_overflow() {
    let _g = fresh();
    let mut s = 0xC0FF_EE11u32;
    let mut injected = 0u32;
    while injected < 100_000 {
        reset();
        assert_eq!(request(OP_LISTEN, &LISTEN_PORT.to_le_bytes()).0, 0);
        let mut conn = connect();
        for _ in 0..250 {
            if !numbers_alive(conn) {
                reset();
                assert_eq!(request(OP_LISTEN, &LISTEN_PORT.to_le_bytes()).0, 0);
                conn = connect();
            }
            {
                injected += 1;
                let seg = hostile(&mut s, conn);
                if xorshift(&mut s).is_multiple_of(16) {
                    let mut bytes = encode(&seg);
                    let at = xorshift(&mut s) as usize % bytes.len();
                    bytes[at] ^= 1 << (xorshift(&mut s) % 8);
                    bytes.truncate(xorshift(&mut s) as usize % (bytes.len() + 1));
                    inject_raw(REMOTE, LOCAL, &bytes);
                } else {
                    inject(seg);
                }
            }
            match xorshift(&mut s) % 16 {
                0 => {
                    let _ = recv(conn);
                }
                1 => {
                    let n = (xorshift(&mut s) % 3000) as usize;
                    let _ = send(conn, &vec![0x41; n.min(crate::protocol::SEGMENT_PAYLOAD_MAX)]);
                }
                2 => {
                    nonos_libc::advance(i64::from(xorshift(&mut s) % 5000));
                    crate::server::retransmit::scan(crate::clock::now_ms());
                }
                3 if xorshift(&mut s).is_multiple_of(8) => {
                    let _ = request(OP_CLOSE, &conn.to_le_bytes());
                }
                _ => crate::peer::drain(),
            }
            bounded();
        }
        crate::peer::drain();
        bounded();
    }
}

fn numbers_alive(conn: u32) -> bool {
    crate::state::TABLE.lock().by_handle_mut(conn).is_some()
}
