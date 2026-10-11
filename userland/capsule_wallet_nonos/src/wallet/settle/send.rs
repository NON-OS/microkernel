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

//! One ASYNC frame to settle's port (v1.4.53): settle's registry pid, the
//! only sender whose answer is believed, once it is sent.

use nonos_libc::{mk_ipc_send, mk_service_lookup};
use nonos_payment_proto::EAGAIN;
use nonos_settle_proto::{request_async, SERVICE};

pub(super) fn send(op: u16, id: u32, body: &[u8]) -> Result<u32, i32> {
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid) != 0 || pid == 0 {
        return Err(EAGAIN);
    }
    let tx = request_async(op, id, body);
    match mk_ipc_send(port as u64, tx.as_ptr(), tx.len()) {
        n if n < 0 => Err(EAGAIN),
        _ => Ok(pid),
    }
}
