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
use nonos_libc::mk_ipc_send;

use crate::protocol::{
    encode_response_header, write_output_status, write_status, OutputStatus, Request,
    KERNEL_REPLY_ENDPOINT, OUTPUT_STATUS_PAYLOAD_LEN, RESP_HDR_LEN, STATUS_LEN,
};

pub fn handle(status: &OutputStatus, req: &Request, tx: &mut [u8]) {
    let body = STATUS_LEN + OUTPUT_STATUS_PAYLOAD_LEN;
    encode_response_header(tx, req, body as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    write_output_status(&mut tx[RESP_HDR_LEN + STATUS_LEN..], status);
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + body);
}
