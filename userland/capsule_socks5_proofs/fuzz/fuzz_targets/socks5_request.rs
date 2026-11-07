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

//! net.socks5's readers of client bytes, compiled from the capsule by
//! socks5_proofs. Neither panics. The request decode names exactly the bytes
//! it was given and refuses only what has no known shape; the handshake
//! machine, fed any bytes in any pieces, sends replies that fit their
//! buffer, moves to relaying on an open and parses nothing after, stays
//! closed once closed, copies no more of a domain than its length, and holds
//! no more than one request's bytes before closing.

#![no_main]

use libfuzzer_sys::fuzz_target;
use socks5_proofs::conn::{Conn, Dest, Event};
use socks5_proofs::request::{ask, Ask};
use socks5_proofs::wire::REPLY_LEN;

// The marker bytes, which the capsule exports only to its own tests.
#[path = "../../../capsule_socks5/src/server/request/marks.rs"]
mod marks;
use marks::{STATUS_ASK, STREAM_BYTES, STREAM_NUMBERED, STREAM_NUMBERED_ON, STREAM_RESET, STREAM_RESET_ON};

/// The most bytes a handshake accumulates: a CONNECT carrying a 255 byte
/// domain. The machine's own constant is private to it.
const ACC_MAX: usize = 262;

fn word(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

fn request(frame: &[u8]) {
    match ask(frame) {
        Some(Ask::Stream(rest)) => {
            assert_eq!(frame[0], STREAM_BYTES);
            assert_eq!(rest, &frame[1..]);
        }
        Some(Ask::Reset) => assert_eq!(frame[0], STREAM_RESET),
        Some(Ask::Status) => assert_eq!(frame[0], STATUS_ASK),
        Some(Ask::Numbered(seq, rest)) => {
            assert_eq!(frame[0], STREAM_NUMBERED);
            assert_eq!(seq, word(&frame[1..5]));
            assert_eq!(rest, &frame[5..]);
        }
        Some(Ask::ResetOn(stream)) => {
            assert_eq!(frame[0], STREAM_RESET_ON);
            assert_eq!(stream, word(&frame[1..5]));
            assert_ne!(stream, 0, "stream 0 is the unnamed one, never named");
        }
        Some(Ask::NumberedOn(stream, seq, rest)) => {
            assert_eq!(frame[0], STREAM_NUMBERED_ON);
            assert_eq!(stream, word(&frame[1..5]));
            assert_ne!(stream, 0, "stream 0 is the unnamed one, never named");
            assert_eq!(seq, word(&frame[5..9]));
            assert_eq!(rest, &frame[9..]);
        }
        None => assert!(
            frame.is_empty()
                || ![STREAM_BYTES, STREAM_RESET, STREAM_NUMBERED, STREAM_RESET_ON, STREAM_NUMBERED_ON, STATUS_ASK]
                    .contains(&frame[0])
                || (frame[0] == STREAM_NUMBERED && frame.len() < 5)
                || (frame[0] == STREAM_RESET_ON && (frame.len() < 5 || frame[1..5] == [0; 4]))
                || (frame[0] == STREAM_NUMBERED_ON && (frame.len() < 9 || frame[1..5] == [0; 4])),
            "a frame of a known shape was refused"
        ),
    }
}

fn handshake(bytes: &[u8]) {
    let mut c = Conn::new();
    let (mut held, mut relaying) = (0usize, false);
    let mut rest = bytes;
    // Each step: a length byte, then that many bytes of stream, so the
    // handshake arrives split the way reads may split it, empty reads too.
    while let [len, tail @ ..] = rest {
        let n = usize::from(*len % 64).min(tail.len());
        let (chunk, more) = tail.split_at(n);
        let (was_relaying, was_closed) = (relaying, c.is_closed());
        let ev = c.on_client(chunk);
        let (relay, close) = (matches!(ev, Event::Relay), matches!(ev, Event::Close));
        let consumed = match ev {
            Event::NeedMore => false,
            Event::ToClient { len, .. } => {
                assert!((1..=REPLY_LEN).contains(&len), "a reply of {len} bytes");
                true
            }
            Event::Open(dest) => {
                assert!(!was_relaying && !was_closed, "an open after the handshake ended");
                relaying = true;
                if let Dest::Domain { name, len, .. } = dest {
                    assert!(name[usize::from(len)..].iter().all(|&b| b == 0), "only the name is copied");
                }
                true
            }
            Event::Relay => {
                assert!(was_relaying, "relaying before an open");
                false
            }
            Event::Close => {
                assert!(c.is_closed(), "a close leaves the connection closed");
                false
            }
        };
        if was_relaying {
            assert!(relay, "a relaying connection parses nothing");
        } else if was_closed {
            assert!(close && c.is_closed(), "a closed connection stays closed");
        } else {
            held += chunk.len();
            assert!(held <= ACC_MAX || c.is_closed(), "a handshake of {held} bytes was held");
        }
        if consumed {
            held = 0;
        }
        rest = more;
    }
}

fuzz_target!(|data: &[u8]| {
    let Some((&pick, body)) = data.split_first() else {
        return;
    };
    match pick % 2 {
        0 => request(body),
        _ => handshake(body),
    }
});
