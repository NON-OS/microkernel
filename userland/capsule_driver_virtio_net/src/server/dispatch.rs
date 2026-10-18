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

//! One request, decoded and handed to its handler.

use crate::protocol::{
    decode_request, E_INVAL, HDR_LEN, OP_HEALTHCHECK, OP_LINK_STATUS, OP_MAC_ADDRESS, OP_RX_BATCH,
    OP_RX_PACKET, OP_TX_PACKET,
};
use crate::server::error::{reply_decode_failed, reply_with_status};
use crate::server::handlers::{self, rx_kept::Kept};
use crate::setup::Driver;

/// Serve `msg` from `pid`. True when the device has stalled and must be
/// reset before the next request.
pub fn serve(
    pid: u32,
    driver: &mut Driver,
    msg: &[u8],
    kept: &mut Option<Kept>,
    tx: &mut [u8],
) -> bool {
    let Some(req) = decode_request(msg) else {
        let _ = reply_decode_failed(pid, tx, E_INVAL);
        return false;
    };
    let body = msg.get(HDR_LEN..).unwrap_or(&[]);
    let _ = match req.op {
        OP_TX_PACKET => return handlers::tx_packet::handle(pid, driver, &req, body, tx),
        OP_HEALTHCHECK => handlers::health::handle(pid, &req, tx),
        OP_LINK_STATUS => handlers::link_status::handle(pid, driver, &req, tx),
        OP_MAC_ADDRESS => handlers::mac_address::handle(pid, driver, &req, tx),
        OP_RX_PACKET => handlers::rx_packet::handle(pid, driver, &req, tx),
        OP_RX_BATCH => handlers::rx_batch::handle(pid, driver, &req, body, kept, tx),
        _ => reply_with_status(pid, tx, &req, E_INVAL),
    };
    false
}
