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

//! Waiting for a command's completion: GET_ENCAPSULATED_RESPONSE polled on
//! the clock (Remote NDIS 1.0, 3.1). RESPONSE_AVAILABLE comes on an
//! interrupt endpoint driver.xhci0 does not configure beside the bulk
//! pipes, so it is not waited for. Only the bytes the device sent are
//! read: the transport reports the true length of the data stage.

use nonos_libc::{mk_idle_ms, Deadline};
use nonos_usbnet::xhci::CONTROL_MAX;
use nonos_usbnet::{Bus, Setup};

use super::from_device::{from_device, FromDevice};
use super::message::{E_REFUSED, E_TIMEDOUT, GET_ENCAPSULATED_RESPONSE, STATUS_SUCCESS};
use super::reply::{answer, Answer};

/// How long a device may take to answer, polled RETRY_MS apart.
const ANSWER_MS: u64 = 1_000;
const RETRY_MS: u64 = 10;

/// The completion of type `kind` under RequestID `id` in `reply`: its
/// MessageLength, or why none came. A media state indicated meanwhile is
/// kept in `media`; anything else that is not the completion is passed over.
pub(super) fn wait<B: Bus>(
    bus: &mut B,
    comm: u8,
    (kind, id): (u32, u32),
    reply: &mut [u8],
    media: &mut Option<bool>,
) -> Result<usize, i32> {
    let get = Setup::class(true, GET_ENCAPSULATED_RESPONSE, 0, comm);
    let room = reply.len().min(CONTROL_MAX);
    let deadline = Deadline::after_ms(ANSWER_MS);
    loop {
        // A STALL here means no answer yet; the deadline bounds it.
        if let Ok(n) = bus.control_in(get, &mut reply[..room]) {
            let sent = &reply[..n.min(room)];
            match answer(sent, kind, id) {
                Answer::Done { status: STATUS_SUCCESS, len } => return Ok(len),
                Answer::Done { .. } => return Err(E_REFUSED),
                Answer::Indication => {
                    if let FromDevice::Media(up) = from_device(sent) {
                        *media = Some(up);
                    }
                }
                Answer::Other => {}
            }
        }
        if deadline.expired() {
            return Err(E_TIMEDOUT);
        }
        mk_idle_ms(RETRY_MS);
    }
}
