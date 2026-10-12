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

//! `OP_CONFIGURE_ISOCH`: add one isochronous endpoint, OUT or IN by its
//! address, to an addressed slot, which holds one of each. Request `slot,
//! ep_address, bInterval, max_burst, mps_le16, mult, 0` (`mps` is
//! wMaxPacketSize whole; `max_burst` and `mult` the SuperSpeed companion's,
//! ignored below it). Reply `dci, 0, 0, 0`.

use super::setup::run;
use crate::error::XhciError;
use crate::protocol::{Request, CONFIGURE_ISOCH_REQUEST_LEN, E_INVAL, E_IO};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::server::handlers::bulk::reply::send_prefix;
use crate::slots::dci_from_ep_address;

pub fn configure(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let ok = body.len() == CONFIGURE_ISOCH_REQUEST_LEN
        && body[1] & 0x70 == 0
        && body[1] & 0x0F != 0
        && (1..=16).contains(&body[2]);
    if !ok {
        return reply_with_status(tx, req, E_INVAL);
    }
    let (dci, mps) = (dci_from_ep_address(body[1]), u16::from_le_bytes([body[4], body[5]]));
    match run(ctx, body[0], dci, body[2], (body[3], body[6]), mps) {
        Ok(()) => send_prefix(tx, req, [dci, 0, 0, 0]),
        Err(XhciError::ControllerUnsupported) => reply_with_status(tx, req, E_INVAL),
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}
