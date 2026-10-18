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

//! `OP_FILL_RANDOM` handler. Bounds the requested length, runs one
//! virtqueue round trip straight into the response buffer, and emits the
//! response. The bytes are wiped from it once sent (plan/50 C8).

use nonos_libc::mk_ipc_send;

use crate::clock::expired_after;
use crate::constants::FILL_MS;
use crate::fill::fill;
use crate::protocol::{
    encode_response_header, write_status, Request, E_IO, E_MSGSIZE, KERNEL_REPLY_ENDPOINT,
    MAX_FILL_BYTES, RESP_HDR_LEN, STATUS_LEN,
};
use crate::scrub::scrub;
use crate::server::error::reply_with_status;
use crate::setup::Driver;

const BODY: usize = RESP_HDR_LEN + STATUS_LEN;

pub fn handle(driver: &mut Driver, req: &Request, tx: &mut [u8]) {
    let want = req.payload_len;
    if want == 0 || want > MAX_FILL_BYTES {
        reply_with_status(tx, req, E_MSGSIZE);
        return;
    }
    let Some(out) = tx.get_mut(BODY..BODY + want as usize) else {
        reply_with_status(tx, req, E_MSGSIZE);
        return;
    };
    let expired = expired_after(FILL_MS);
    let Ok(take) = fill(driver.transport, &mut driver.queue, out, expired) else {
        reply_with_status(tx, req, E_IO);
        return;
    };
    encode_response_header(tx, req, (STATUS_LEN + take) as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), BODY + take);
    scrub(&mut tx[BODY..]);
}
