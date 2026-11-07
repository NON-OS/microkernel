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

//! The NVGP framing, compiled from nonos_gfx_wire by gfx_wire_proofs. None
//! panics. A request decodes to exactly the payload its header declares,
//! never past 256 bytes or the frame, and its header encodes back to the
//! bytes it came from; a reply is held to the request it answers, its body
//! no longer than allowed and inside the frame; an output notice gives only
//! a reason it names.

#![no_main]

use gfx_wire_proofs::header::{Header, HDR_LEN, MAX_REPLY_BODY, MAX_REQUEST};
use gfx_wire_proofs::notice::{why, LEAVING, OUTPUTS_CHANGED};
use gfx_wire_proofs::reply::Reply;
use gfx_wire_proofs::request::Request;
use libfuzzer_sys::fuzz_target;

fn request(frame: &[u8]) {
    let Ok(r) = Request::decode(frame) else {
        return;
    };
    let end = HDR_LEN + r.payload.len();
    assert!(end <= MAX_REQUEST && end <= frame.len(), "a payload past its bound");
    assert_eq!(r.payload.len(), r.header.payload_len as usize);
    assert_eq!(r.payload, &frame[HDR_LEN..end]);
    let mut back = [0u8; HDR_LEN];
    assert!(r.header.encode(&mut back).is_ok());
    assert_eq!(&back[..], &frame[..HDR_LEN], "a header reads back to other bytes");
}

fn reply(bytes: &[u8]) {
    // The request it answers, then the reply.
    let Some((asked, frame)) = bytes.split_at_checked(HDR_LEN) else {
        return;
    };
    let Ok(asked) = Header::decode(asked) else {
        return;
    };
    let Ok(r) = Reply::decode(frame, &asked) else {
        return;
    };
    assert_eq!((r.header.op, r.header.output, r.header.request_id), (asked.op, asked.output, asked.request_id));
    assert!(r.body.len() <= MAX_REPLY_BODY, "a reply body past the limit");
    assert_eq!(r.body.len() + 4, r.header.payload_len as usize);
    assert_eq!(r.body, &frame[HDR_LEN + 4..HDR_LEN + 4 + r.body.len()]);
}

fn notice(body: &[u8]) {
    if let Some(w) = why(body) {
        assert_eq!(body.len(), 4);
        assert!(w == OUTPUTS_CHANGED || w == LEAVING);
    }
}

fuzz_target!(|data: &[u8]| {
    let Some((&pick, body)) = data.split_first() else {
        return;
    };
    match pick % 3 {
        0 => request(body),
        1 => reply(body),
        _ => notice(body),
    }
});
