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

use crate::driver::Driver;
use crate::protocol::{
    E_BAD_OP, E_INVAL, OP_DEVICE_INFO, OP_DMA_STATE, OP_HEALTHCHECK, OP_RF_STATE,
};
use crate::server::{handlers, respond};

pub(super) fn dispatch(
    driver: &mut Driver,
    sender_pid: u32,
    req: crate::protocol::Request,
    body: &[u8],
    tx: &mut [u8],
) {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender_pid, &req, tx),
        OP_DEVICE_INFO if body.is_empty() => handlers::device::handle(driver, sender_pid, &req, tx),
        OP_RF_STATE if body.is_empty() => handlers::rf::handle(driver, sender_pid, &req, tx),
        OP_DMA_STATE if body.is_empty() => handlers::dma::handle(driver, sender_pid, &req, tx),
        _ if body.is_empty() => {
            let _ = respond::send(sender_pid, &req, E_BAD_OP, &[], tx);
        }
        _ => {
            let _ = respond::send(sender_pid, &req, E_INVAL, &[], tx);
        }
    }
}
