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

//! OP_GET_COMMERCE: what the market says the release sells, and whether
//! both its publisher and its payee stand behind it.

use alloc::vec;

use nonos_libc::mk_ipc_call_timeout;
use nonos_market_proto::{pair_body, parse_commerce, reply_body, request, OP_GET_COMMERCE};
use nonos_market_proto::{ReplyError, SERVICE};
use nonos_nox_meter::pay::Offer;
use nonos_payment_proto::{EAGAIN, EINVAL};

use super::lookup::port_of;

const TIMEOUT_MS: u64 = 2000;
const REPLY_MAX: usize = 8192;

pub(crate) fn offer(listing: &[u8], release: &[u8]) -> Result<Offer, i32> {
    let port = port_of(SERVICE).ok_or(EAGAIN)?;
    let id = nonos_libc::mk_time_millis() as u32;
    let tx =
        request(OP_GET_COMMERCE, id, &pair_body(listing, release).ok_or(EINVAL)?).ok_or(EINVAL)?;
    let mut rx = vec![0u8; REPLY_MAX];
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if n <= 0 {
        return Err(EAGAIN);
    }
    let body = match reply_body(&rx[..n as usize], OP_GET_COMMERCE, id) {
        Ok(b) => b,
        Err(ReplyError::Status(s)) => return Err(s),
        Err(_) => return Err(EAGAIN),
    };
    let c = parse_commerce(body).ok_or(EINVAL)?;
    Ok(Offer {
        trusted: c.trusted(),
        listing_id: c.listing_id,
        release_hash: c.release_hash,
        chain_id: c.chain_id,
        token: c.token_contract,
        block: c.block,
    })
}
