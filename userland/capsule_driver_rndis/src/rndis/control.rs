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

//! The RNDIS control channel (Remote NDIS 1.0, 3.1): a message out as
//! SEND_ENCAPSULATED_COMMAND, then its completion waited for (wait.rs).

use nonos_usbnet::{Bus, Setup};

use super::message::{le32, put32, COMPLETION, SEND_ENCAPSULATED_COMMAND};
use super::wait::wait;

pub struct Control {
    pub comm: u8,
    next_id: u32,
    /// The last media state indicated while a command waited, so the bind
    /// starts from it rather than from up.
    pub media: Option<bool>,
}

impl Control {
    pub fn new(comm: u8) -> Self {
        Self { comm, next_id: 1, media: None }
    }

    /// Send `msg` under a fresh RequestID and wait for its completion in
    /// `reply`, at most CONTROL_MAX bytes asked for; its MessageLength,
    /// never more than the bytes the device sent.
    pub fn command<B: Bus>(
        &mut self,
        bus: &mut B,
        msg: &mut [u8],
        reply: &mut [u8],
    ) -> Result<usize, i32> {
        // RequestID 0 is never used, so no completion can match a HALT.
        let id = self.next_id;
        self.next_id = id.wrapping_add(1).max(1);
        put32(msg, 8, id);
        let kind = le32(msg, 0).unwrap_or(0) | COMPLETION;
        bus.control_out(Setup::class(false, SEND_ENCAPSULATED_COMMAND, 0, self.comm), msg)?;
        wait(bus, self.comm, (kind, id), reply, &mut self.media)
    }
}
