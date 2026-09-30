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

use crate::constants::{MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME};
use crate::protocol::{Request, E_AGAIN, E_INVAL, E_MSGSIZE, MAX_TX_PAYLOAD_BYTES};
use crate::server::error::reply_with_status;
use crate::setup::Driver;
use crate::tx::{busy, send};

pub fn handle(sender: u32, driver: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) {
    if req.payload_len as usize != body.len() {
        reply_with_status(sender, tx, req, E_MSGSIZE);
        return;
    }
    if body.len() < MIN_ETHERNET_FRAME
        || body.len() > MAX_ETHERNET_FRAME
        || body.len() as u32 > MAX_TX_PAYLOAD_BYTES
    {
        reply_with_status(sender, tx, req, E_INVAL);
        return;
    }
    if busy(driver) {
        reply_with_status(sender, tx, req, E_AGAIN);
        return;
    }
    send(driver, body);
    reply_with_status(sender, tx, req, 0);
}
