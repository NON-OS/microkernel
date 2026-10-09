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

use crate::protocol::{
    Header, E_BAD_LEN, E_INVAL, OP_GET_CHUNK, OP_GET_COUNT, OP_GET_SIZE, OP_GET_SLUG,
};

use super::handlers::{op_get_chunk, op_get_count, op_get_size, op_get_slug};
use super::respond;

/// Every path answers once: the caller is blocked in its call until the reply
/// comes, and one never sent costs it the whole timeout. A frame too short to
/// hold a header names no op or index, so its refusal is sent under zeros.
pub fn serve(sender: u32, frame: &[u8]) {
    let Some(hdr) = Header::decode(frame) else {
        respond::err(sender, 0, 0, E_BAD_LEN);
        return;
    };
    match hdr.op {
        OP_GET_COUNT => op_get_count::handle(sender),
        OP_GET_SIZE => op_get_size::handle(sender, hdr.index),
        OP_GET_CHUNK => op_get_chunk::handle(sender, hdr.index, hdr.offset),
        OP_GET_SLUG => op_get_slug::handle(sender, hdr.index),
        _ => respond::err(sender, hdr.op, hdr.index, E_INVAL),
    }
}
