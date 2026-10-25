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

//! One request as it arrived, and how it is answered: payment_proto's
//! header, echoing the op, flags and request id.

use alloc::vec::Vec;

use nonos_payment_proto::{reply, status_reply, EOVERFLOW};

pub(crate) struct Call<'a> {
    pub op: u16,
    pub flags: u16,
    pub id: u32,
    pub body: &'a [u8],
    /// The kernel-stamped pid the call came from.
    pub sender: u32,
}

impl Call<'_> {
    /// A body too long to frame is refused with EOVERFLOW, never sent under a
    /// length it does not have.
    pub fn answer(&self, status: i32, body: &[u8]) -> Vec<u8> {
        reply(self.op, self.flags, self.id, status, body)
            .unwrap_or_else(|| status_reply(self.op, self.flags, self.id, EOVERFLOW))
    }

    /// The body, or the status, as one reply.
    pub fn result(&self, r: Result<Vec<u8>, i32>) -> Vec<u8> {
        match r {
            Ok(body) => self.answer(0, &body),
            Err(status) => self.answer(status, &[]),
        }
    }
}
