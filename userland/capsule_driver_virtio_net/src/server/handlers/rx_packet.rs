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

//! OP_RX_PACKET: one received frame, as status, a u32 length and the frame.
//! The frame is copied straight into the reply and wiped from it once sent.

use crate::constants::{MAX_ETHERNET_FRAME, Q_RX};
use crate::protocol::{
    encode_response_header, write_status, Request, E_AGAIN, E_IO, RESP_HDR_LEN, RX_FRAME_AT,
    RX_PAYLOAD_PREFIX_LEN, STATUS_LEN,
};
use crate::rx::take_one;
use crate::scrub::scrub;
use crate::server::error::{reply, reply_with_status};
use crate::setup::Driver;

pub fn handle(sender_pid: u32, driver: &mut Driver, req: &Request, tx: &mut [u8]) -> bool {
    // Room for the longest frame take_one hands on, or no frame is taken.
    let Some(frame) = tx.get_mut(RX_FRAME_AT..RX_FRAME_AT + MAX_ETHERNET_FRAME) else {
        return reply_with_status(sender_pid, tx, req, E_IO);
    };
    let taken = take_one(&mut driver.rx, frame);
    driver.transport.notify(Q_RX);
    let Some(len) = taken else {
        return reply_with_status(sender_pid, tx, req, E_AGAIN);
    };
    // The buffer is at least RX_FRAME_AT + MAX_ETHERNET_FRAME long (checked
    // above) and take_one never returns more than it was given.
    encode_response_header(tx, req, (STATUS_LEN + RX_PAYLOAD_PREFIX_LEN + len) as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    tx[RESP_HDR_LEN + STATUS_LEN..RX_FRAME_AT].copy_from_slice(&(len as u32).to_le_bytes());
    let sent = reply(sender_pid, tx, RX_FRAME_AT + len);
    scrub(&mut tx[RX_FRAME_AT..RX_FRAME_AT + len]);
    sent
}
