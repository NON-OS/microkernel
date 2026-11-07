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

//! net.nym's readers of gateway and client bytes, compiled from the capsule
//! by nym_reply_proofs. None panics. A fragment header reads back to the
//! bytes it came from; unpadding returns a prefix of its input; a WebSocket
//! frame never claims more bytes than the buffer holds, and its payload fits
//! where it was copied; the request decode hands back exactly the payload
//! its header declares;
//! reply fragments in any order, repeated or forged, never make the
//! reassembly hold more sets or bytes than its bounds, and whatever it
//! completes reads as a reply or as nothing.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nym_reply_proofs::message::{parse, unpad, UNLINKED_HEADER_LEN};
use nym_reply_proofs::reply::{reply_message, Collected, Reassembly, MAX_PENDING, MESSAGE_MAX, PENDING_BYTES_MAX};
use nym_reply_proofs::server::parse_req;
use nym_reply_proofs::ws_frame::parse::next;
use nym_reply_proofs::ws_frame::types::FrameKind;

fn fragment(bytes: &[u8]) {
    if let Some((f, payload)) = parse(bytes) {
        assert_eq!(payload.len() + UNLINKED_HEADER_LEN, bytes.len());
        assert!(f.current >= 1 && f.current <= f.total);
        assert_eq!(f.into_bytes(payload), bytes, "a fragment reads back to other bytes");
    }
}

fn padding(bytes: &[u8]) {
    if let Some(message) = unpad(bytes) {
        assert!(message.len() < bytes.len() && bytes.starts_with(message));
        assert_eq!(bytes[message.len()], 1);
    }
}

fn frames(bytes: &[u8], room: usize) {
    let mut out = vec![0u8; room];
    let mut ctrl = [0u8; 125];
    let mut at = 0usize;
    while at < bytes.len() {
        match next(&bytes[at..], &mut out, &mut ctrl) {
            Ok(Some(frame)) => {
                assert!(frame.consumed > 0 && frame.consumed <= bytes.len() - at);
                match frame.kind {
                    FrameKind::Binary | FrameKind::Text => assert!(frame.len <= out.len()),
                    FrameKind::Ping => assert!(frame.len <= ctrl.len()),
                    _ => {}
                }
                at += frame.consumed;
            }
            Ok(None) | Err(_) => return,
        }
    }
}

fn request(bytes: &[u8]) {
    if let Ok((_, payload)) = parse_req::parse(bytes) {
        // Exactly the length the header declares, from the end of the header;
        // anything after it is not read.
        let declared = u32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]) as usize;
        assert_eq!(payload, &bytes[parse_req::HDR_LEN..parse_req::HDR_LEN + declared]);
    }
}

fn reassembly(bytes: &[u8]) {
    let mut r = Reassembly::new();
    let mut rest = bytes;
    let mut now = 0i64;
    // Each step: a length byte, a clock step byte, then that many bytes of
    // fragment, so sets arrive interleaved and time moves between them.
    while let [len, step, tail @ ..] = rest {
        let n = usize::from(*len).min(tail.len());
        let (fragment, more) = tail.split_at(n);
        now += i64::from(*step) * 50;
        if let Collected::Complete(message) = r.collect(fragment, now) {
            assert!(message.len() <= MESSAGE_MAX, "a message past the limit");
            let _ = reply_message(&message);
        }
        assert!(r.pending() <= MAX_PENDING, "more sets held than allowed");
        assert!(r.held_bytes() <= PENDING_BYTES_MAX, "more bytes held than allowed");
        rest = more;
    }
}

fuzz_target!(|data: &[u8]| {
    let Some((&pick, body)) = data.split_first() else {
        return;
    };
    match pick % 5 {
        0 => fragment(body),
        1 => padding(body),
        2 => frames(body, usize::from(pick >> 2) * 64),
        3 => request(body),
        _ => reassembly(body),
    }
});
