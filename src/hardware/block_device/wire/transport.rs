// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use alloc::vec::Vec;

use super::codec::{decode_response, encode_request};
use super::error::WireError;
use super::status::lift;
use super::target::Target;
use crate::services::lifecycle::transport::{self, TransportError};

/// Send `op` for the target's unit with `body` and wait for its reply; the
/// payload on success, the lifted status otherwise.
pub(super) fn call(t: Target, op: u16, body: &[u8]) -> Result<Vec<u8>, WireError> {
    let e = t.e;
    let asked = crate::sys::timer::uptime_ms();
    let _one = transport::lock_yielding(&e.slot.call);
    let sent = crate::sys::timer::uptime_ms();
    let request_id = transport::next_request_id(&e.slot.seq);
    let frame = encode_request(op, t.unit, request_id, body);
    let resp = transport::round_trip(
        request_id,
        &frame,
        e.sender,
        e.reply_inbox,
        e.state,
        decode_response,
    )
    .map_err(|err| {
        super::slow::lost(e, op, sent - asked, crate::sys::timer::uptime_ms() - asked);
        map_err(err)
    })?;
    let now = crate::sys::timer::uptime_ms();
    super::slow::say(e, op, resp.status, sent - asked, now - asked);
    if resp.status != 0 {
        return Err(lift(resp.status));
    }
    Ok(resp.body)
}

fn map_err(e: TransportError) -> WireError {
    match e {
        TransportError::Dead => WireError::Dead,
        TransportError::Stale => WireError::Stale,
        TransportError::TransportFailure => WireError::TransportFailure,
        TransportError::ProtocolMismatch => WireError::ProtocolMismatch,
    }
}
