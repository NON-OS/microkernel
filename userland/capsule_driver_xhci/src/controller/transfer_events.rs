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

//! What a transfer's events say, pure: whether one stopped the transfer
//! before its last TRB, how the last one completed, and how short its data
//! stage came. The data stage asks for a Short Packet event (ISP), whose
//! third word's bits 23:0 are the bytes it did not fill (xHCI 1.2 4.10.1.1
//! and 6.4.2.1, as recalled, unverified), so the bytes a device sent are
//! known, never taken to be the bytes asked for.

use crate::constants::{CC_SHORT_PACKET, CC_SUCCESS};
use crate::error::{XhciError, XhciResult};
use crate::rings::event::IssuedTransfer;
use crate::trb::Trb;

pub fn complete(code: u8) -> XhciResult<()> {
    match code {
        CC_SUCCESS | CC_SHORT_PACKET => Ok(()),
        other => Err(XhciError::TransferCompletionFailed(other)),
    }
}

/// Whether `event` is on one of `earlier`, on the same slot and endpoint
/// as `issued`.
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

/// The residual a Short Packet event on one of `earlier` reports.
pub fn short_residual(event: &Trb, issued: IssuedTransfer, earlier: &[u64]) -> Option<u32> {
    let short = event.completion_code() == CC_SHORT_PACKET && on_earlier(event, issued, earlier);
    short.then_some(event.d2 & 0x00FF_FFFF)
}

/// The bytes a data stage of `asked` brought in: the residual clamped to
/// it, so a residual past the request reads as nothing, never as more.
pub fn received(asked: u16, residual: u32) -> u16 {
    asked - residual.min(u32::from(asked)) as u16
}
