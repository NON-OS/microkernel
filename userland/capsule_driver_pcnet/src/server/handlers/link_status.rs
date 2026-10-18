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

//! `OP_LINK_STATUS`: BCR4.LEDOUT, which follows link status out of reset on
//! both parts, sampled on every call.

use crate::constants::csr::{BCR4, BCR4_LEDOUT};
use crate::protocol::{
    encode_response_header, write_status, Request, LINK_STATUS_PAYLOAD_LEN, RESP_HDR_LEN,
    STATUS_LEN,
};
use crate::report;
use crate::server::error::reply;
use crate::setup::Driver;

pub fn handle(sender: u32, driver: &Driver, req: &Request, tx: &mut [u8]) -> bool {
    let bcr4 = driver.regs.bcr(BCR4);
    report::link(bcr4);
    let payload_len = STATUS_LEN + LINK_STATUS_PAYLOAD_LEN;
    encode_response_header(tx, req, payload_len as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    tx[RESP_HDR_LEN + STATUS_LEN] = (bcr4 & BCR4_LEDOUT != 0) as u8;
    reply(sender, tx, RESP_HDR_LEN + payload_len)
}
