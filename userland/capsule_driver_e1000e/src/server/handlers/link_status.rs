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

//! `OP_LINK_STATUS`: STATUS.LU, read live, as one byte (1 up, 0 down). A
//! change from the last state logged is logged once, with speed and duplex
//! when the link is up, so the console shows the cable going in and out.

use crate::constants::regs::REG_STATUS;
use crate::constants::status::STATUS_LU;
use crate::protocol::{
    encode_response_header, write_status, Request, LINK_STATUS_PAYLOAD_LEN, RESP_HDR_LEN,
    STATUS_LEN,
};
use crate::server::error::reply;
use crate::server::link_log::log_change;
use crate::setup::Driver;

pub fn handle(sender: u32, driver: &mut Driver, req: &Request, tx: &mut [u8]) {
    // SAFETY: `driver.regs` is the broker-mapped BAR0 window; STATUS is a
    // 4-byte register inside it.
    let status = unsafe { driver.regs.r32(REG_STATUS) };
    let up = status & STATUS_LU != 0;
    if driver.link_logged != Some(up) {
        driver.link_logged = Some(up);
        log_change(status);
    }
    let payload_len = (STATUS_LEN + LINK_STATUS_PAYLOAD_LEN) as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    tx[RESP_HDR_LEN + STATUS_LEN] = up as u8;
    reply(sender, tx, RESP_HDR_LEN + STATUS_LEN + LINK_STATUS_PAYLOAD_LEN);
}
