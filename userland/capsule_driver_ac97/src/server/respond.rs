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

//! Replies go to the sender's call; only the audio server can call here
//! (the kernel holds the endpoint to it).

use nonos_libc::mk_ipc_reply;

use crate::codec::Ident;
use crate::protocol::{
    response_header, write_output_status, write_status, Request, HDR_LEN, OUTPUT_STATUS_LEN,
    STATUS_LEN,
};

pub fn status(sender: u32, req: &Request, errno: i32, tx: &mut [u8]) {
    response_header(tx, req, STATUS_LEN as u32);
    write_status(tx, errno);
    let _ = mk_ipc_reply(sender, tx.as_ptr(), HDR_LEN + STATUS_LEN);
}

pub fn output(sender: u32, req: &Request, id: &Ident, tx: &mut [u8]) {
    let body = STATUS_LEN + OUTPUT_STATUS_LEN;
    response_header(tx, req, body as u32);
    write_status(tx, 0);
    write_output_status(&mut tx[HDR_LEN + STATUS_LEN..], id);
    let _ = mk_ipc_reply(sender, tx.as_ptr(), HDR_LEN + body);
}
