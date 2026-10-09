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

//! Any bytes a client can send net.socks5, through the real request decode
//! the serving loop reads every frame with (`ask`) and the real handshake
//! machine the stream bytes of a fresh connection go to. The loop answers
//! every frame whatever `ask` makes of it, so what is proven here is that
//! neither step panics or reads out of range on any input, that `ask` names
//! exactly the bytes it was given, and that the machine keeps the promises
//! its loop relies on: a reply fits its buffer, an open moves to relaying, a
//! close stays closed, and a handshake cannot grow past one request.

use crate::conn::{Conn, Dest, Event};
use crate::request::{
    ask, Ask, STATUS_ASK, STREAM_BYTES, STREAM_NUMBERED, STREAM_NUMBERED_ON, STREAM_RESET,
    STREAM_RESET_ON,
};
use crate::wire::{ATYP_DOMAIN, ATYP_IPV4, ATYP_IPV6, CMD_CONNECT, METHOD_NONE, REPLY_LEN, VER};

const FUZZ_ROUNDS: usize = 200_000;
/// The most bytes a handshake accumulates: a CONNECT carrying a 255 byte
/// domain. The machine's own constant is private to it.
const ACC_MAX: usize = 262;

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

/// What must hold for any frame `ask` is given.
fn check_ask(frame: &[u8]) {
    match ask(frame) {
        Some(Ask::Stream(rest)) => {
            assert_eq!(frame[0], STREAM_BYTES);
            assert_eq!(rest, &frame[1..]);
        }
        Some(Ask::Reset) => assert_eq!(frame[0], STREAM_RESET),
        Some(Ask::Status) => assert_eq!(frame[0], STATUS_ASK),
        Some(Ask::Numbered(seq, rest)) => {
            assert_eq!(frame[0], STREAM_NUMBERED);
            assert!(frame.len() >= 5);
            assert_eq!(seq, u32::from_le_bytes([frame[1], frame[2], frame[3], frame[4]]));
            assert_eq!(rest, &frame[5..]);
        }
        Some(Ask::ResetOn(stream)) => {
            assert_eq!(frame[0], STREAM_RESET_ON);
            assert_eq!(stream, u32::from_le_bytes([frame[1], frame[2], frame[3], frame[4]]));
            assert_ne!(stream, 0, "stream 0 is the unnamed one, never named");
        }
        Some(Ask::NumberedOn(stream, seq, rest)) => {
            assert_eq!(frame[0], STREAM_NUMBERED_ON);
            assert!(frame.len() >= 9);
            assert_eq!(stream, u32::from_le_bytes([frame[1], frame[2], frame[3], frame[4]]));
            assert_ne!(stream, 0, "stream 0 is the unnamed one, never named");
            assert_eq!(seq, u32::from_le_bytes([frame[5], frame[6], frame[7], frame[8]]));
            assert_eq!(rest, &frame[9..]);
        }
        None => assert!(
            frame.is_empty()
                || ![STREAM_BYTES, STREAM_RESET, STREAM_NUMBERED, STREAM_RESET_ON, STREAM_NUMBERED_ON, STATUS_ASK]
                    .contains(&frame[0])
                || (frame[0] == STREAM_NUMBERED && frame.len() < 5)
                || (frame[0] == STREAM_RESET_ON && (frame.len() < 5 || frame[1..5] == [0; 4]))
                || (frame[0] == STREAM_NUMBERED_ON && (frame.len() < 9 || frame[1..5] == [0; 4])),
            "a frame of a known shape was refused: {frame:?}"
        ),
    }
}

/// Feed `chunks` to a fresh connection, checking each step against what the
/// serving loop relies on. Returns the last event's kind.
fn drive(chunks: &[&[u8]]) -> &'static str {
    drive_counting(chunks).0
}

/// `drive`, also counting the replies the connection sent on the way.
fn drive_counting(chunks: &[&[u8]]) -> (&'static str, usize) {
    let mut c = Conn::new();
    let mut held = 0usize;
    let mut kind = "none";
    let mut replies = 0usize;
    for chunk in chunks {
        let was_relaying = c.is_relaying();
        let was_closed = c.is_closed();
        let ev = c.on_client(chunk);
        kind = match ev {
            Event::NeedMore => "more",
            Event::ToClient { len, .. } => {
                assert!((1..=REPLY_LEN).contains(&len), "a reply of {len} bytes");
                replies += 1;
                "reply"
            }
            Event::Open(dest) => {
                assert!(c.is_relaying(), "an open moves to relaying");
                if let Dest::Domain { name, len, .. } = dest {
                    assert!(name[usize::from(len)..].iter().all(|&b| b == 0), "only the name is copied");
                }
                "open"
            }
            Event::Relay => "relay",
            Event::Close => {
                assert!(c.is_closed(), "a close leaves the connection closed");
                "close"
            }
        };
        if was_relaying {
            assert_eq!(kind, "relay", "a relaying connection parses nothing");
        } else if was_closed {
            assert_eq!(kind, "close", "a closed connection stays closed");
        } else {
            held += chunk.len();
            if held > ACC_MAX {
                assert_eq!(kind, "close", "a handshake of {held} bytes was held");
            }
        }
        if kind == "reply" || kind == "open" {
            // What a reply or an open consumed is no longer held.
            held = 0;
        }
    }
    (kind, replies)
}

/// A handshake shaped enough to reach the request phase most of the time.
fn handshake(s: &mut u64) -> Vec<u8> {
    let r = xorshift(s);
    let mut b = Vec::new();
    // Greeting: the version, a method count and that many methods.
    let n = match r % 4 {
        0 => (xorshift(s) % 256) as u8,
        _ => (xorshift(s) % 4) as u8 + 1,
    };
    b.push(if (r >> 2) & 7 == 0 { xorshift(s) as u8 } else { VER });
    b.push(n);
    for i in 0..n {
        let offer = (r >> 5) & 3 != 0 && i == 0;
        b.push(if offer { METHOD_NONE } else { xorshift(s) as u8 });
    }
    // Request: version, command, reserved, an address type and its address.
    b.push(if (r >> 8) & 7 == 0 { xorshift(s) as u8 } else { VER });
    b.push(if (r >> 11) & 7 == 0 { xorshift(s) as u8 } else { CMD_CONNECT });
    b.push(0);
    match (r >> 14) % 5 {
        0 => {
            b.push(ATYP_IPV4);
            b.extend((0..4).map(|_| xorshift(s) as u8));
        }
        1 => {
            b.push(ATYP_IPV6);
            b.extend((0..16).map(|_| xorshift(s) as u8));
        }
        2 | 3 => {
            let len = match (r >> 17) % 4 {
                0 => 0,
                1 => 255,
                _ => (xorshift(s) % 64) as u8,
            };
            b.push(ATYP_DOMAIN);
            b.push(len);
            b.extend((0..len).map(|_| b'a' + (xorshift(s) % 26) as u8));
        }
        _ => b.push(xorshift(s) as u8),
    }
    b.extend((0..2).map(|_| xorshift(s) as u8));
    // Whatever follows: stream bytes once relaying, junk before.
    let tail = (xorshift(s) % 48) as usize;
    b.extend((0..tail).map(|_| xorshift(s) as u8));
    if (r >> 20) & 7 == 0 {
        b.truncate((xorshift(s) % (b.len() as u64 + 1)) as usize);
    }
    if (r >> 23) & 7 == 0 && !b.is_empty() {
        let i = (xorshift(s) % b.len() as u64) as usize;
        b[i] ^= 1 << (xorshift(s) % 8);
    }
    b
}

#[test]
fn random_frames_decode_to_exactly_what_they_carry() {
    let mut s = 0x534F_434B_0000_0001u64;
    for _ in 0..FUZZ_ROUNDS {
        let r = xorshift(&mut s);
        let len = (xorshift(&mut s) % 40) as usize;
        let mut f: Vec<u8> = (0..len).map(|_| xorshift(&mut s) as u8).collect();
        if !f.is_empty() && r & 3 != 0 {
            f[0] = (r >> 8) as u8 % 4;
        }
        check_ask(&f);
    }
}

#[test]
fn random_handshakes_in_random_pieces_keep_the_machine_promises() {
    let mut s = 0x534F_434B_0000_0002u64;
    let (mut opened, mut refused, mut closed) = (0usize, 0usize, 0usize);
    for _ in 0..FUZZ_ROUNDS {
        let b = handshake(&mut s);
        // Split it into pieces, sometimes empty, the way reads may split it.
        let mut cuts = Vec::new();
        let mut at = 0usize;
        while at < b.len() {
            let step = match xorshift(&mut s) % 4 {
                0 => 0,
                1 => 1,
                _ => (xorshift(&mut s) % 40) as usize + 1,
            };
            let end = (at + step).min(b.len());
            cuts.push(&b[at..end]);
            at = end;
        }
        // An empty read after the last piece lets a pipelined request run.
        cuts.push(&[]);
        cuts.push(&[]);
        match drive_counting(&cuts) {
            ("open" | "relay", _) => opened += 1,
            ("close", replies) => {
                closed += 1;
                // A greeting reply then a refusal of the request.
                if replies == 2 {
                    refused += 1;
                }
            }
            _ => {}
        }
    }
    assert!(opened > FUZZ_ROUNDS / 10, "the generator reaches an open: {opened}");
    assert!(refused > 1000 && closed > FUZZ_ROUNDS / 10, "and the refusals: {refused} {closed}");
}

#[test]
fn boundary_requests() {
    // The request decode: empty, each marker with nothing after it, and a
    // numbered frame one byte short of its number.
    for f in [&[][..], &[STREAM_BYTES][..], &[STREAM_RESET][..], &[STREAM_NUMBERED, 1, 2, 3][..], &[3][..], &[0xFF][..]] {
        check_ask(f);
    }
    assert!(ask(&[]).is_none());
    assert!(matches!(ask(&[STREAM_NUMBERED, 1, 0, 0, 0]), Some(Ask::Numbered(1, rest)) if rest.is_empty()));
    let big = vec![STREAM_BYTES; 34 * 1024];
    assert!(matches!(ask(&big), Some(Ask::Stream(rest)) if rest.len() == big.len() - 1));

    // The machine: nothing yet, a wrong version, every method count.
    assert_eq!(drive(&[&[][..]]), "more");
    assert_eq!(drive(&[&[4][..]]), "close");
    for n in [0u8, 1, 254, 255] {
        let mut g = vec![VER, n];
        g.extend(core::iter::repeat_n(METHOD_NONE, n as usize));
        let kind = drive(&[&g[..g.len() - 1]]);
        assert!(n == 0 || kind == "more", "a greeting one short of {n} methods");
        drive(&[&g[..]]);
    }
    // Each address type, and a domain of no and of the longest length.
    let greet = [VER, 1, METHOD_NONE];
    let v4 = [VER, CMD_CONNECT, 0, ATYP_IPV4, 10, 0, 0, 1, 0, 80];
    assert_eq!(drive(&[&greet[..], &v4[..]]), "open");
    let mut v6 = vec![VER, CMD_CONNECT, 0, ATYP_IPV6];
    v6.extend([0u8; 18]);
    assert_eq!(drive(&[&greet[..], &v6[..]]), "open");
    for len in [0u8, 1, 255] {
        let mut d = vec![VER, CMD_CONNECT, 0, ATYP_DOMAIN, len];
        d.extend(core::iter::repeat_n(b'a', len as usize));
        d.extend([1, 187]);
        let kind = drive(&[&greet[..], &d[..]]);
        assert!(len == 0 || kind == "open", "a domain of {len} bytes: {kind}");
        drive(&[&greet[..], &d[..d.len() - 1]]);
    }
    // An unknown address type and an unknown command are refused.
    assert_ne!(drive(&[&greet[..], &[VER, CMD_CONNECT, 0, 9, 0, 0][..]]), "open");
    assert_ne!(drive(&[&greet[..], &[VER, 2, 0, ATYP_IPV4, 1, 2, 3, 4, 0, 80][..]]), "open");
    // A handshake that fills the accumulator exactly is held; one byte more
    // closes it.
    let fill = [VER; ACC_MAX];
    assert_eq!(drive(&[&[VER, 255][..]]), "more");
    let mut over = vec![VER, 255];
    over.extend([1u8; ACC_MAX - 1]);
    assert_eq!(drive(&[&over[..]]), "close");
    drive(&[&fill[..]]);
}
