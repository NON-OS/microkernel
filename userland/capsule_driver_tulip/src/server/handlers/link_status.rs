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

//! `OP_LINK_STATUS`. Neither emulated part has a PHY to ask: the 21140's
//! general-purpose port and the 21143's SIA status mean what the board
//! makes them mean. A virtual card is cabled for as long as it runs, so the
//! link is up while both processes are, sampled on every call.

use crate::constants::csr::CSR5;
use crate::init::running;
use crate::protocol::{
    encode_response_header, write_status, Request, LINK_STATUS_PAYLOAD_LEN, RESP_HDR_LEN,
    STATUS_LEN,
};
use crate::report;
use crate::server::error::reply;
use crate::setup::Driver;

pub fn handle(sender: u32, driver: &Driver, req: &Request, tx: &mut [u8]) -> bool {
    let up = running(driver.regs.csr(CSR5));
    report::link(up);
    let payload_len = STATUS_LEN + LINK_STATUS_PAYLOAD_LEN;
    encode_response_header(tx, req, payload_len as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    tx[RESP_HDR_LEN + STATUS_LEN] = up as u8;
    reply(sender, tx, RESP_HDR_LEN + payload_len)
}
