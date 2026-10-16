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

//! DELIVERED (op 12, v1.4.46): an order's key from the escrow's Delivered
//! log, for the meter's ORDER_DELIVERED with orderId alone. Read by
//! address and topic0 only, in pages from the escrow's deploy_block; the
//! orderId is filtered here, never asked of the node. Each chain's reading
//! is kept (DeliveredKeys), so a later ask reads only new blocks; past
//! MAX_KEYS a miss rescans for its own order. EAGAIN while no log names it.

use alloc::vec::Vec;

use nonos_nox_config::{deployed, Contract};
use nonos_settle_proto::{parse_chain_hash, DeliveredKeys};
use nonos_settle_proto::{EAGAIN, EINVAL, ENOENT};

use super::delivered_scan::scan;
use crate::settle::Settle;

pub fn delivered(s: &mut Settle, body: &[u8]) -> Result<Vec<u8>, i32> {
    let ask = parse_chain_hash(body).ok_or(EINVAL)?;
    let chain = ask.chain_id;
    let escrow = s.live.usable(&mut s.rpc, chain, Contract::Escrow)?;
    let first = deployed(chain, Contract::Escrow).map_err(|_| ENOENT)?.block;
    if first == 0 {
        return Err(ENOENT);
    }
    let at = match s.delivered.iter().position(|k| k.chain_id == chain) {
        Some(at) => at,
        None => {
            let at = s.delivered.len();
            s.delivered.push(DeliveredKeys::new(chain));
            at
        }
    };
    if let Some(key) = s.delivered.get(at).ok_or(ENOENT)?.find(&ask.id) {
        return Ok(key.to_vec());
    }
    let head = crate::chain::head(&mut s.rpc, chain)?;
    let keys = s.delivered.get_mut(at).ok_or(ENOENT)?;
    let found = match keys.full {
        false => {
            let from = keys.scanned.saturating_add(1).max(first);
            let more = scan(&mut s.rpc, chain, &escrow, (from, head), &ask.id)?;
            keys.absorb(more.clone(), head);
            more
        }
        true => scan(&mut s.rpc, chain, &escrow, (first, head), &ask.id)?,
    };
    let key = found.iter().find(|(o, _)| *o == ask.id).map(|(_, k)| *k);
    key.map(|k| k.to_vec()).ok_or(EAGAIN)
}
