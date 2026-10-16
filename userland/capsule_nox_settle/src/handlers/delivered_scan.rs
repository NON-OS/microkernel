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

//! The escrow's Delivered logs over a block range, by address and topic0
//! only (plan/26), in pages of LOG_PAGE_BLOCKS, read strictly: (orderId,
//! key) for each. Past MAX_KEYS pairs the cache is given up anyway, so only
//! the first pair naming the asked order is kept after that: a node cannot
//! grow the answer past MAX_KEYS + 2, and what the ask finds is the same.

use alloc::vec::Vec;

use nonos_eth_rpc::{parse_logs, request_logs};
use nonos_settle_proto::{delivered as from_log, log_pages, DELIVERED_TOPIC, EPROTO, MAX_KEYS};

use crate::net::Rpc;

type Pairs = Vec<([u8; 32], [u8; 32])>;

pub(super) fn scan(
    rpc: &mut Rpc,
    chain: u64,
    escrow: &[u8; 20],
    (from, to): (u64, u64),
    ask: &[u8; 32],
) -> Result<Pairs, i32> {
    let mut out: Pairs = Vec::new();
    for (a, b) in log_pages(chain, from, to)? {
        let resp = rpc.call(chain, &request_logs(escrow, &DELIVERED_TOPIC, a, b, 1))?;
        let logs = parse_logs(&resp).ok_or(EPROTO)?;
        for pair in logs
            .iter()
            .filter(|l| l.address == *escrow)
            .filter_map(|l| from_log(&l.topics, &l.data))
        {
            if out.len() <= MAX_KEYS || (pair.0 == *ask && !out.iter().any(|p| p.0 == *ask)) {
                out.push(pair);
            }
        }
    }
    Ok(out)
}
