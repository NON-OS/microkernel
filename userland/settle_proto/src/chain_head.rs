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
//! The head a log read pages to, judged before any page is asked for
//! (plan/26): two hosts name about the same block, and the range from the
//! scanned-to mark to it lies inside the chain's cap, SERVICE_MS of its
//! blocks. The cap is a backstop; the mark keeps an honest range small.

use nonos_nox_config::Chain;

use crate::{ENOENT, EPROTO};

/// 20 years of 365.25 days, in ms: how long past a contract's deploy block
/// settle still reads its logs whole.
pub const SERVICE_MS: u64 = 631_152_000_000;

/// How far apart in time two hosts' heads may be: one has seen a few blocks
/// the other has not yet.
pub const HEAD_SKEW_MS: u64 = 300_000;

/// How many of `chain`'s blocks `ms` holds.
pub const fn blocks_in(chain: Chain, ms: u64) -> u64 {
    match ms.checked_div(chain.block_ms()) {
        Some(n) => n,
        None => 0,
    }
}

/// The head two hosts' answers confirm: the lower, so neither can move it
/// up alone, and only while they are HEAD_SKEW_MS of blocks apart or less.
pub fn confirmed_head(chain: Chain, first: u64, second: u64) -> Option<u64> {
    (first.abs_diff(second) <= blocks_in(chain, HEAD_SKEW_MS)).then_some(first.min(second))
}

/// The range [from, to] a log read on `chain_id` may page: EPROTO past
/// SERVICE_MS of the chain's blocks, ENOENT on a chain the config lacks.
pub fn log_range(chain_id: u64, from: u64, to: u64) -> Result<(u64, u64), i32> {
    let chain = Chain::from_id(chain_id).ok_or(ENOENT)?;
    match to.saturating_sub(from) <= blocks_in(chain, SERVICE_MS) {
        true => Ok((from, to)),
        false => Err(EPROTO),
    }
}
