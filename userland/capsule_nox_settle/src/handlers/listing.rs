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

//! LISTING (op 11, v1.4.16): a listing's current releases from the config
//! registry's logs. The chain's cursor reads only blocks it has not seen;
//! a cursor that is full rescans for the asked listing alone. ENOENT when
//! the registry is not deployed, failed its checks, or has no deploy_block.

use alloc::vec::Vec;

use nonos_nox_config::{deployed, Contract};
use nonos_settle_proto::{current, listing_reply, parse_chain_hash, EFBIG, EINVAL, ENOENT};

use crate::listing::scan;
use crate::settle::Settle;

pub fn listing(s: &mut Settle, body: &[u8]) -> Result<Vec<u8>, i32> {
    let ask = parse_chain_hash(body).ok_or(EINVAL)?;
    let chain = ask.chain_id;
    let registry = s.live.usable(&mut s.rpc, chain, Contract::ListingRegistry)?;
    let first = deployed(chain, Contract::ListingRegistry).map_err(|_| ENOENT)?.block;
    if first == 0 {
        return Err(ENOENT);
    }
    let head = crate::chain::head(&mut s.rpc, chain)?;
    let cursor = s.cursors.get(chain).ok_or(ENOENT)?;
    if !cursor.full {
        let from = cursor.scanned.saturating_add(1).max(first);
        let more = scan(&mut s.rpc, chain, &registry, (from, head), None)?;
        cursor.absorb(more, head);
    }
    let rows = match cursor.full {
        false => current(&ask.id, &cursor.found.published, &cursor.found.withdrawn),
        true => {
            let own = scan(&mut s.rpc, chain, &registry, (first, head), Some(&ask.id))?;
            current(&ask.id, &own.published, &own.withdrawn)
        }
    };
    listing_reply(&rows).ok_or(EFBIG)
}
