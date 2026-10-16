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

//! The pages of a log read over [from, to]. `to` is the head a node named,
//! so a range past LOG_PAGES_MAX pages is refused (EPROTO), never paged.

use nonos_eth_rpc::{block_pages, LOG_PAGE_BLOCKS};
use nonos_settle_proto::EPROTO;

/// 20.48M blocks: years of either chain past a contract's deploy_block.
const LOG_PAGES_MAX: u64 = 2048;

pub fn log_pages(from: u64, to: u64) -> Result<impl Iterator<Item = (u64, u64)>, i32> {
    if to.saturating_sub(from) / LOG_PAGE_BLOCKS >= LOG_PAGES_MAX {
        return Err(EPROTO);
    }
    Ok(block_pages(from, to, LOG_PAGE_BLOCKS))
}
