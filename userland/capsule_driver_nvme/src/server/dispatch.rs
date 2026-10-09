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

//! One decoded request routed to the handler for its op.

use crate::protocol::{
    Request, E_INVAL, OP_CAPACITY, OP_CONTROLLER_INFO, OP_FLUSH, OP_HEALTHCHECK,
    OP_IDENTIFY_CONTROLLER, OP_IDENTIFY_NAMESPACE, OP_READ_BLOCKS, OP_SMART_HEALTH,
    OP_WRITE_BLOCKS,
};
use crate::server::{error, handlers};
use crate::setup::Driver;

pub(super) fn dispatch(driver: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_HEALTHCHECK
        | OP_CONTROLLER_INFO
        | OP_IDENTIFY_CONTROLLER
        | OP_IDENTIFY_NAMESPACE
        | OP_SMART_HEALTH
            if req.payload_len != 0 =>
        {
            error::reply_with_status(tx, req, E_INVAL)
        }
        OP_HEALTHCHECK => handlers::health::handle(req, tx),
        OP_CONTROLLER_INFO => handlers::controller_info::handle(driver, req, tx),
        OP_IDENTIFY_CONTROLLER => handlers::identify_controller::handle(driver, req, tx),
        OP_IDENTIFY_NAMESPACE => handlers::identify_namespace::handle(driver, req, tx),
        OP_SMART_HEALTH => handlers::smart_health::handle(driver, req, tx),
        OP_CAPACITY => handlers::capacity::handle(driver, req, tx),
        OP_READ_BLOCKS => handlers::read::handle(driver, req, body, tx),
        OP_WRITE_BLOCKS => handlers::write::handle(driver, req, body, tx),
        OP_FLUSH => handlers::flush::handle(driver, req, tx),
        _ => error::reply_with_status(tx, req, E_INVAL),
    }
}
