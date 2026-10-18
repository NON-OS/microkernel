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

use crate::protocol::{
    Request, E_INVAL, OP_CAPACITY, OP_FLUSH, OP_HEALTHCHECK, OP_READ_BLOCKS, OP_WRITE_BLOCKS,
};
use crate::server::error::reply_with_status;
use crate::server::handlers;
use crate::setup::Driver;

/// One request, by op. Any other op (DISCARD and WRITE_ZEROES among them,
/// which this driver does not serve: the plan names them their own unit) is
/// answered E_INVAL and never reaches the device.
pub fn dispatch(driver: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_HEALTHCHECK => handlers::health::handle(req, tx),
        OP_CAPACITY => handlers::capacity::handle(driver, req, tx),
        OP_READ_BLOCKS => handlers::read::handle(driver, req, body, tx),
        OP_WRITE_BLOCKS => handlers::write::handle(driver, req, body, tx),
        OP_FLUSH => handlers::flush::handle(driver, req, tx),
        _ => {
            let _ = reply_with_status(tx, req, E_INVAL);
        }
    }
}
