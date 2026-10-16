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
//! The pages of a log read over [from, to]. `to` is a head two hosts
//! confirmed (head.rs), and a range past the chain's cap is refused
//! (EPROTO, settle_proto's log_range), never paged.

use nonos_eth_rpc::{block_pages, LOG_PAGE_BLOCKS};
use nonos_settle_proto::log_range;

pub fn log_pages(
    chain_id: u64,
    from: u64,
    to: u64,
) -> Result<impl Iterator<Item = (u64, u64)>, i32> {
    let (from, to) = log_range(chain_id, from, to)?;
    Ok(block_pages(from, to, LOG_PAGE_BLOCKS))
}
