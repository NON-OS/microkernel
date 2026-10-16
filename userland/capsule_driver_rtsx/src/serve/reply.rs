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

//! The reply: the request's header echoed, the status, then the payload
//! the handler already put in place. The kernel's own client (pid 0) takes
//! it from the fixed reply endpoint by request id; a capsule caller gets it
//! matched to its call, as the USB storage driver answers.

use nonos_libc::{mk_ipc_reply, mk_ipc_send};

use crate::protocol::{encode_reply, Request, E_OK, HDR_LEN, KERNEL_REPLY_ENDPOINT, STATUS_LEN};

/// Send the answer; the bytes of `tx` the reply used.
pub fn send(sender: u32, req: &Request, answer: Result<usize, i32>, tx: &mut [u8]) -> usize {
    let (status, body) = match answer {
        Ok(n) => (E_OK, n),
        Err(status) => (status, 0),
    };
    let len = HDR_LEN + STATUS_LEN + body;
    let Some(head) = tx.get_mut(..HDR_LEN + STATUS_LEN) else { return 0 };
    let Ok(head) = <&mut [u8; HDR_LEN + STATUS_LEN]>::try_from(head) else { return 0 };
    encode_reply(head, req, status, body as u32);
    let Some(frame) = tx.get(..len) else { return 0 };
    let _ = if sender == 0 {
        mk_ipc_send(KERNEL_REPLY_ENDPOINT, frame.as_ptr(), frame.len())
    } else {
        mk_ipc_reply(sender, frame.as_ptr(), frame.len())
    };
    len
}
