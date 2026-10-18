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

use crate::protocol::{
    encode_response_header, write_status, Request, E_IO, KERNEL_REPLY_ENDPOINT, RESP_HDR_LEN,
    STATUS_LEN,
};
use crate::server::error::reply_with_status;
use crate::setup::Driver;
use nonos_libc::mk_ipc_send;

const DATA_AT: usize = RESP_HDR_LEN + STATUS_LEN;

/// The answered read's sectors, from the data buffer. A buffer the queue
/// will not hand over (a request still holds it) or a reply that does not
/// fit is answered E_IO, never with whatever the buffer held.
pub(super) fn reply_read(driver: &mut Driver, req: &Request, tx: &mut [u8], bytes_n: usize) {
    let end = DATA_AT.saturating_add(bytes_n);
    let taken = tx.get_mut(DATA_AT..end).is_some_and(|out| driver.queue.take(out));
    if !taken {
        reply_with_status(tx, req, E_IO);
        return;
    }
    encode_response_header(tx, req, (STATUS_LEN + bytes_n) as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), end);
}
