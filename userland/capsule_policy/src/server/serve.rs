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

//! One received frame, dispatched to its handler or refused.
//!
//! The loop only receives; everything a frame's bytes decide happens here,
//! so the proofs drive the same path the service runs.

use nonos_policy_proto::{decode_field, Header, E_BAD_LEN, E_INVAL, HDR_LEN, OP_GET, OP_SET};

use super::{handle_get, handle_set, respond};

/// Every path answers once: the caller is blocked in its call until the reply
/// comes, and one never sent costs it the whole timeout. A frame too short to
/// hold a header names no op or field, so its refusal is sent under zeros.
pub fn serve(sender: u32, frame: &[u8]) {
    let Some(hdr) = Header::decode(frame) else {
        respond::err(sender, 0, 0, 0, E_BAD_LEN);
        return;
    };
    let body = frame.get(HDR_LEN..).and_then(|rest| rest.get(..usize::from(hdr.payload_len)));
    let Some(body) = body else {
        respond::err(sender, hdr.op, hdr.field, hdr.kind, E_INVAL);
        return;
    };
    let Some(field) = decode_field(hdr.field) else {
        respond::err(sender, hdr.op, hdr.field, hdr.kind, E_INVAL);
        return;
    };
    match hdr.op {
        OP_GET => handle_get::dispatch(sender, field),
        OP_SET => handle_set::dispatch(sender, field, body),
        _ => respond::err(sender, hdr.op, hdr.field, hdr.kind, E_INVAL),
    }
}
