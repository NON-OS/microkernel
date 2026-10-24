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
//! `OP_CONFIGURE_STREAMS` (0x24): `slot, pair, dirs, log2`. Bit 0 of
//! `dirs` names the pair's IN endpoint, bit 1 its OUT; each is given 2^log2
//! stream contexts (log2 1 to 4, within the controller's MaxPSASize) by
//! dropping and adding it again in one Configure Endpoint. SuperSpeed only,
//! once an endpoint, before any transfer on it. Reply `usable le16, 0, 0`:
//! the stream ids BULK_IN and BULK_OUT may then name, 1 to that.

use super::reply::send_prefix;
use super::streams_give::give;
use crate::controller::MAX_LOG2;
use crate::error::XhciError;
use crate::protocol::{Request, E_INVAL, E_IO};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::slots::pair_of;

pub fn configure_streams(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let &[slot, pair, dirs @ 1..=3, log2] = body else {
        return reply_with_status(tx, req, E_INVAL);
    };
    let psa = ctx.driver.layout.max_psa;
    let Some(second) =
        pair_of(pair).filter(|_| psa != 0 && log2 >= 1 && log2 <= MAX_LOG2.min(psa + 1))
    else {
        return reply_with_status(tx, req, E_INVAL);
    };
    let mut usable = 0u16;
    for dir_in in [true, false].into_iter().filter(|&d| dirs & if d { 1 } else { 2 } != 0) {
        usable = match give(ctx, (slot, second, dir_in), log2) {
            Ok(n) => n,
            Err(XhciError::ControllerUnsupported) => return reply_with_status(tx, req, E_INVAL),
            Err(_) => return reply_with_status(tx, req, E_IO),
        };
    }
    send_prefix(tx, req, [usable as u8, (usable >> 8) as u8, 0, 0]);
}
