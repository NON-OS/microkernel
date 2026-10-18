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

//! What a Transfer Event says about a transfer of several TRBs, for
//! `wait_transfer_completion`.

use crate::constants::{CC_SHORT_PACKET, CC_SUCCESS};
use crate::error::{XhciError, XhciResult};
use crate::rings::event::IssuedTransfer;
use crate::trb::Trb;

/// The transfer's answer from its last TRB's completion code.
pub fn complete(code: u8) -> XhciResult<()> {
    match code {
        CC_SUCCESS | CC_SHORT_PACKET => Ok(()),
        other => Err(XhciError::TransferCompletionFailed(other)),
    }
}

/// Whether `event` names one of `earlier` on the endpoint `issued` is on.
fn on_earlier(event: &Trb, issued: IssuedTransfer, earlier: &[u64]) -> bool {
    let at = event.get_pointer() & !0xF;
    event.slot_id() == issued.slot
        && event.endpoint_id() == issued.dci
        && earlier.iter().any(|&p| p & !0xF == at)
}

/// Whether `event` is an error on one of `earlier`, on the same slot and
/// endpoint as `issued`.
pub fn stopped_early(event: &Trb, issued: IssuedTransfer, earlier: &[u64]) -> bool {
    let cc = event.completion_code();
    cc != CC_SUCCESS && cc != CC_SHORT_PACKET && on_earlier(event, issued, earlier)
}

/// The bytes one of `earlier` left untransferred, when `event` is its
/// Short Packet: the TRB Transfer Length field, 24 bits.
pub fn short_on(event: &Trb, issued: IssuedTransfer, earlier: &[u64]) -> Option<u32> {
    let short = event.completion_code() == CC_SHORT_PACKET && on_earlier(event, issued, earlier);
    short.then_some(event.d2 & 0x00FF_FFFF)
}
