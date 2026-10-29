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

//! A named stream's frames: the reset and the numbered exchange a tunnel
//! builds for stream 0, renamed on the way out (`frame_on`), and read back
//! by net.socks5's own parser as that stream's and no other's.

use crate::frame::{numbered, reset, FIRST_SEQ, NUMBERED, RESET};
use crate::frame_on::{on, NUMBERED_ON, RESET_ON, STREAMS_PER_CALLER};
use crate::socks5_request::ask::{ask, Ask};

#[test]
fn stream_0_goes_as_it_is_built_and_is_read_as_the_unnamed_conversation() {
    let f = numbered(FIRST_SEQ, b"hello").unwrap_or_default();
    assert_eq!(on(0, &reset()), None);
    assert_eq!(on(0, &f), None);
    assert!(matches!(ask(&f), Some(Ask::Numbered(FIRST_SEQ, b)) if b == b"hello"));
    assert!(matches!(ask(&[0, 9]), Some(Ask::Stream(b)) if b == [9]));
}

#[test]
fn a_reset_names_its_stream_and_nothing_else() {
    for s in 1..STREAMS_PER_CALLER {
        let f = on(s, &reset()).unwrap_or_default();
        assert_eq!(f[..], [RESET_ON, s as u8, 0, 0, 0]);
        assert!(matches!(ask(&f), Some(Ask::ResetOn(got)) if got == s), "stream {s}");
    }
}

#[test]
fn a_numbered_exchange_keeps_its_number_and_bytes_on_its_stream() {
    for s in [1u32, 2, 7, 0x0102_0304] {
        for (seq, body) in [(FIRST_SEQ, &b""[..]), (9, b"\x05\x01\x00"), (u32::MAX, &[0u8; 300])] {
            let f = on(s, &numbered(seq, body).unwrap_or_default()).unwrap_or_default();
            assert_eq!(f[0], NUMBERED_ON);
            assert_eq!(f.len(), 9 + body.len());
            let Some(Ask::NumberedOn(got_s, got_seq, got_body)) = ask(&f) else {
                panic!("net.socks5 reads stream {s} seq {seq} as something else");
            };
            assert_eq!((got_s, got_seq, got_body), (s, seq, body));
        }
    }
}

#[test]
fn nothing_but_a_reset_or_an_exchange_is_renamed() {
    for marker in 0u8..=255 {
        let got = on(3, &[marker, 1, 2, 3, 4]);
        assert_eq!(got.is_some(), marker == RESET || marker == NUMBERED, "marker {marker}");
    }
    assert_eq!(on(3, &[]), None);
}

/* The markers are net.socks5's and net.anon's, and below the SOCKS version. */
const _: () = assert!(RESET_ON == 3 && NUMBERED_ON == 4 && STREAMS_PER_CALLER == 24);
