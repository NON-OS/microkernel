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
use super::get_config_descriptor::get_config_descriptor;
use super::reply::reply_config;
use crate::protocol::{Request, CONFIG_DESCRIPTOR_REQUEST_LEN, CONTROL_DATA_MAX, E_INVAL, E_IO};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::server::handlers::recover::{recover_after, DCI_EP0};

/// `OP_GET_CONFIG_DESCRIPTOR` (`slot, index, len_le16`): configuration
/// `index`, up to `len` bytes and at most one page; the reply carries the
/// bytes the device sent, prefixed with their count.
pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != CONFIG_DESCRIPTOR_REQUEST_LEN || body[0] == 0 {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let len = u16::from_le_bytes([body[2], body[3]]).min(CONTROL_DATA_MAX);
    if len == 0 {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let out = match ctx.driver.dma_pool.alloc(len as u64) {
        Ok(r) => r,
        Err(_) => {
            reply_with_status(tx, req, E_IO);
            return;
        }
    };
    out.zero();
    match get_config_descriptor(ctx, body[0], body[1], &out, len) {
        Ok(actual) => reply_config(tx, req, &out, actual),
        Err(e) => {
            recover_after(ctx, body[0], DCI_EP0, e, Some(out));
            reply_with_status(tx, req, E_IO)
        }
    }
}
