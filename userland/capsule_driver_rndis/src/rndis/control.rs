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

//! The RNDIS control channel (Remote NDIS 1.0, 3.1; Linux rndis_command):
//! a message out as SEND_ENCAPSULATED_COMMAND, then GET_ENCAPSULATED_RESPONSE
//! polled on the clock until its completion comes. RESPONSE_AVAILABLE comes
//! on an interrupt endpoint driver.xhci0 does not configure beside the bulk
//! pipes, so it is not waited for; Linux polls without it too.

use nonos_libc::{mk_idle_ms, Deadline};
use nonos_usbnet::xhci::CONTROL_MAX;
use nonos_usbnet::{Bus, Setup};

use super::message::{le32, put32, COMPLETION, E_REFUSED, E_TIMEDOUT, STATUS_SUCCESS};
use super::message::{GET_ENCAPSULATED_RESPONSE, SEND_ENCAPSULATED_COMMAND};
use super::reply::{answer, Answer};

/// Linux tries ten times 40 ms apart; a second on the clock covers that.
const ANSWER_MS: u64 = 1_000;
const RETRY_MS: u64 = 10;

pub struct Control {
    pub comm: u8,
    next_id: u32,
}

impl Control {
    pub fn new(comm: u8) -> Self {
        Self { comm, next_id: 1 }
    }

    /// Send `msg` under a fresh RequestID and wait for its completion in
    /// `reply`, at most CONTROL_MAX bytes asked for; its MessageLength.
    pub fn command<B: Bus>(
        &mut self,
        bus: &mut B,
        msg: &mut [u8],
        reply: &mut [u8],
    ) -> Result<usize, i32> {
        // RequestID 0 is never used: Linux skips it, and HALT carries it.
        let id = self.next_id;
        self.next_id = id.wrapping_add(1).max(1);
        put32(msg, 8, id);
        let kind = le32(msg, 0).unwrap_or(0) | COMPLETION;
        bus.control_out(Setup::class(false, SEND_ENCAPSULATED_COMMAND, 0, self.comm), msg)?;
        let get = Setup::class(true, GET_ENCAPSULATED_RESPONSE, 0, self.comm);
        let room = reply.len().min(CONTROL_MAX);
        let deadline = Deadline::after_ms(ANSWER_MS);
        loop {
            // A STALL here means no answer yet; Linux ignores it the same way.
            if let Ok(n) = bus.control_in(get, &mut reply[..room]) {
                if let Answer::Done { status, len } = answer(&reply[..n.min(room)], kind, id) {
                    return if status == STATUS_SUCCESS { Ok(len) } else { Err(E_REFUSED) };
                }
            }
            if deadline.expired() {
                return Err(E_TIMEDOUT);
            }
            mk_idle_ms(RETRY_MS);
        }
    }
}
