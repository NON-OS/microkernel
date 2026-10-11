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

//! Which handler answers which op.

use crate::protocol::{
    Request, E_INVAL, OP_HEALTHCHECK, OP_LINK_STATUS, OP_MAC_ADDRESS, OP_RX_PACKET, OP_STATS,
    OP_TX_PACKET,
};
use crate::server::error::reply_with_status;
use crate::server::handlers;
use crate::setup::Driver;

pub fn dispatch(sender: u32, driver: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_HEALTHCHECK => handlers::health::handle(sender, req, tx),
        OP_LINK_STATUS => handlers::link_status::handle(sender, driver, req, tx),
        OP_MAC_ADDRESS => handlers::mac_address::handle(sender, driver, req, tx),
        OP_TX_PACKET => handlers::tx_packet::handle(sender, driver, req, body, tx),
        OP_RX_PACKET => handlers::rx_packet::handle(sender, driver, req, tx),
        OP_STATS => handlers::stats::handle(sender, driver, req, tx),
        _ => reply_with_status(sender, tx, req, E_INVAL),
    }
}
