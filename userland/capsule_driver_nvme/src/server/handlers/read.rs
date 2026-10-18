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
    encode_response_header, write_status, Request, E_MSGSIZE, E_NODEV, KERNEL_REPLY_ENDPOINT,
    READ_REQ_LEN, RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

pub fn handle(driver: &mut Driver, req: &Request, sender: u32, body: &[u8], tx: &mut [u8]) {
    let io = match driver.io.as_ref() {
        Some(i) => i,
        None => return reply_with_status(tx, req, E_NODEV),
    };
    if req.payload_len as usize != READ_REQ_LEN {
        return reply_with_status(tx, req, E_MSGSIZE);
    }
    let (lba, nsectors) = match super::rw_parse::parse(body, io.capacity_sectors, io.max_sectors())
    {
        Ok(v) => v,
        Err(s) => return reply_with_status(tx, req, s),
    };
    let bytes = nsectors as usize * io.lba_size as usize;
    driver.io_command(
        sender,
        tx,
        |io, regs, tx| io.read_into(regs, lba, nsectors, &mut tx[RESP_HDR_LEN + STATUS_LEN..]),
        |tx, done| match done {
            Ok(()) => reply_with_blocks(tx, req, bytes),
            Err(status) => reply_with_status(tx, req, status),
        },
    );
}

/// Status 0 and the `bytes` read, which `tx` holds after its header.
fn reply_with_blocks(tx: &mut [u8], req: &Request, bytes: usize) {
    let payload = STATUS_LEN + bytes;
    encode_response_header(tx, req, payload as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + payload);
}
