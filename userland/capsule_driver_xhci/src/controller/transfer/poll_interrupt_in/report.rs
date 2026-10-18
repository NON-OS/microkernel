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
use super::poll::IntrPoll;
use crate::constants::{CC_SHORT_PACKET, CC_SUCCESS};
use crate::error::{XhciError, XhciResult};
use crate::trb::Trb;
/// The report the event completes, or the error it ended in.
pub(super) fn completed(event: &Trb, length: u16) -> XhciResult<IntrPoll> {
    match event.completion_code() {
        CC_SUCCESS | CC_SHORT_PACKET => Ok(IntrPoll::Complete(transferred(event, length))),
        cc => Err(XhciError::TransferCompletionFailed(cc)),
    }
}
/// The bytes the report holds: the request less the residual the event
/// reports, the residual clamped to the request. A residual past the request
/// says more bytes stayed behind than were asked for, so none of the buffer
/// is vouched for and none is handed up, as the bulk path reads it too.
fn transferred(event: &Trb, length: u16) -> u16 {
    let residual = event.d2 & 0x00FF_FFFF;
    let req = length as u32;
    (req - residual.min(req)) as u16
}
