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

//! The registry's Published and Withdrawn logs over a block range, by
//! address and topic0 only, in pages of LOG_PAGE_BLOCKS, read
//! strictly. `only` keeps one listing's rows, filtered here, never asked of
//! the node.

use alloc::vec::Vec;

use nonos_eth_rpc::{parse_logs, request_logs};
use nonos_settle_proto::{withdrawn, Listed, EPROTO, PUBLISHED_TOPIC, WITHDRAWN_TOPIC};

use super::cursor::MAX_ROWS;
use crate::chain::log_pages;
use crate::net::Rpc;

#[derive(Default)]
pub(crate) struct Found {
    pub published: Vec<Listed>,
    pub withdrawn: Vec<([u8; 32], [u8; 32])>,
}

impl Found {
    pub fn rows(&self) -> usize {
        self.published.len().saturating_add(self.withdrawn.len())
    }
}

pub(crate) fn scan(
    rpc: &mut Rpc,
    chain_id: u64,
    registry: &[u8; 20],
    (from, to): (u64, u64),
    only: Option<&[u8; 32]>,
) -> Result<Found, i32> {
    let mut out = Found::default();
    let keep = |listing: &[u8; 32]| only.is_none_or(|o| o == listing);
    for (a, b) in log_pages(chain_id, from, to)? {
        for topic in [PUBLISHED_TOPIC, WITHDRAWN_TOPIC] {
            let resp = rpc.call(chain_id, &request_logs(registry, &topic, a, b, 1))?;
            for l in parse_logs(&resp).ok_or(EPROTO)?.iter().filter(|l| l.address == *registry) {
                if let Some(p) =
                    Listed::from_log(&l.topics, &l.data, l.block).filter(|p| keep(&p.listing_id))
                {
                    out.published.push(p);
                } else if let Some(w) = withdrawn(&l.topics, &l.data).filter(|w| keep(&w.0)) {
                    out.withdrawn.push(w);
                }
            }
        }
        /* Past MAX_ROWS the cursor gives its rows up and rescans for one listing. */
        if only.is_none() && out.rows() > MAX_ROWS {
            break;
        }
    }
    Ok(out)
}
