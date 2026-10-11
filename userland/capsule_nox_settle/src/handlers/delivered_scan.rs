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
//! only, in pages of LOG_PAGE_BLOCKS, read strictly: (orderId,
//! key) for each.

use alloc::vec::Vec;

use nonos_eth_rpc::{block_pages, parse_logs, request_logs, LOG_PAGE_BLOCKS};
use nonos_settle_proto::{delivered as from_log, DELIVERED_TOPIC, EPROTO};

use crate::net::Rpc;

type Pairs = Vec<([u8; 32], [u8; 32])>;

pub(super) fn scan(
    rpc: &mut Rpc,
    chain: u64,
    escrow: &[u8; 20],
    (from, to): (u64, u64),
) -> Result<Pairs, i32> {
    let mut out = Vec::new();
    for (a, b) in block_pages(from, to, LOG_PAGE_BLOCKS) {
        let resp = rpc.call(chain, &request_logs(escrow, &DELIVERED_TOPIC, a, b, 1))?;
        let logs = parse_logs(&resp).ok_or(EPROTO)?;
        out.extend(
            logs.iter()
                .filter(|l| l.address == *escrow)
                .filter_map(|l| from_log(&l.topics, &l.data)),
        );
    }
    Ok(out)
}
