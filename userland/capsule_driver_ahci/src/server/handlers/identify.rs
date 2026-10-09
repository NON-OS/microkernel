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

use crate::constants::ata::SECTOR_SIZE;
use crate::protocol::{
    encode_identify, encode_response_header, write_status, Request, E_NODEV, IDENTIFY_PAYLOAD_LEN,
    KERNEL_REPLY_ENDPOINT, MEDIUM_SATA, RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

/// OP_IDENTIFY: the served disk's size, sector size, model and serial, as
/// its IDENTIFY block gave them at bring-up (`protocol::identify_reply`).
pub fn handle(driver: &Driver, req: &Request, tx: &mut [u8]) {
    let port = match driver.block.as_ref() {
        Some(p) => p,
        None => return reply_with_status(tx, req, E_NODEV),
    };
    let payload = STATUS_LEN + IDENTIFY_PAYLOAD_LEN;
    encode_response_header(tx, req, payload as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    encode_identify(
        &mut tx[RESP_HDR_LEN + STATUS_LEN..],
        port.capacity_sectors,
        SECTOR_SIZE as u32,
        port.names.model(),
        port.names.serial(),
        MEDIUM_SATA,
    );
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + payload);
}
