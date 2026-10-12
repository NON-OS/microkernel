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

//! The SynIC through the kernel, for the VMBus root this core holds.

use nonos_libc::{mk_hv_post, mk_hv_recv, MK_HV_MESSAGE_BYTES};

use crate::constants::{MESSAGE_TYPE_CHANNEL, POST_MS};
use crate::wait;

const E_AGAIN: i64 = -11;
const HEADER: usize = 16;

/// A host message's payload, held to the 240 bytes a slot carries.
pub struct Msg {
    pub len: usize,
    pub data: [u8; MK_HV_MESSAGE_BYTES - HEADER],
}

impl Msg {
    pub fn bytes(&self) -> &[u8] {
        &self.data[..self.len]
    }
}

#[derive(Clone, Copy)]
pub struct Hv {
    pub device_id: u64,
}

impl Hv {
    /// A channel message to the host, waiting a while for room.
    pub fn post(&self, connection: u32, payload: &[u8]) -> Result<(), &'static str> {
        let rc = wait::until(POST_MS, || {
            let rc = mk_hv_post(
                self.device_id,
                connection,
                MESSAGE_TYPE_CHANNEL,
                payload.as_ptr(),
                payload.len(),
            );
            (rc != E_AGAIN).then_some(rc)
        });
        match rc {
            Some(0) => Ok(()),
            Some(_) => Err("message refused"),
            None => Err("host had no room for a message"),
        }
    }

    /// The oldest VMBus message waiting. A message of another type is taken
    /// and dropped: VMBus's SINT carries none.
    pub fn recv(&self) -> Option<Msg> {
        let mut raw = [0u8; MK_HV_MESSAGE_BYTES];
        let n = mk_hv_recv(self.device_id, &mut raw);
        if n < HEADER as i64 {
            return None;
        }
        let ty = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
        if ty != MESSAGE_TYPE_CHANNEL {
            return None;
        }
        let len = (n as usize).min(MK_HV_MESSAGE_BYTES) - HEADER;
        let mut data = [0u8; MK_HV_MESSAGE_BYTES - HEADER];
        data[..len].copy_from_slice(&raw[HEADER..HEADER + len]);
        Some(Msg { len, data })
    }
}
