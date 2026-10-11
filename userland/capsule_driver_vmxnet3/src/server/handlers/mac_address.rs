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

//! `OP_MAC_ADDRESS`: the address drawn at bring-up and written to MACL and
//! MACH, which is the one the device filters on.

use crate::protocol::{
    encode_response_header, write_status, Request, MAC_ADDRESS_PAYLOAD_LEN, RESP_HDR_LEN,
    STATUS_LEN,
};
use crate::server::error::reply;
use crate::setup::Driver;

pub fn handle(sender: u32, driver: &Driver, req: &Request, tx: &mut [u8]) -> bool {
    let payload_len = STATUS_LEN + MAC_ADDRESS_PAYLOAD_LEN;
    encode_response_header(tx, req, payload_len as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let at = RESP_HDR_LEN + STATUS_LEN;
    tx[at..at + MAC_ADDRESS_PAYLOAD_LEN].copy_from_slice(&driver.mac);
    reply(sender, tx, RESP_HDR_LEN + payload_len)
}
