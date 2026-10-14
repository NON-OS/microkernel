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

//! The fees a transaction offers, from the network's own recent blocks: the
//! next block's base fee, and the tips the last blocks' transactions paid,
//! as `eth_feeHistory` reports them. The tip is the median of those blocks'
//! median tips, the cap twice the next base fee and the tip, so a base fee
//! that doubles before inclusion still fits. Nothing is a fixed gwei level:
//! at a base fee under one gwei the old floor of a one-gwei tip paid more in
//! tip than in fee. Pure, so wallet_proofs reads a real answer with it.

use alloc::vec::Vec;

#[path = "fees_scan.rs"]
mod fees_scan;
use fees_scan::{array_after, quantities};

/// The least tip offered, 0.01 gwei: a block builder may pass over none at all.
pub const MIN_TIP_WEI: u128 = 10_000_000;

/// The next block's base fee and each block's median tip, from one
/// `eth_feeHistory` answer asked with the 50th percentile only.
pub fn parse_fee_history(obj: &[u8]) -> Option<(u128, Vec<u128>)> {
    if obj.windows(7).any(|w| w == b"\"error\"") {
        return None;
    }
    let bases = quantities(array_after(obj, b"\"baseFeePerGas\"")?);
    let next = *bases.last()?;
    let tips = quantities(array_after(obj, b"\"reward\"")?);
    Some((next, tips))
}

/// The tip and the cap per gas, in wei.
pub fn fees(next_base: u128, mut tips: Vec<u128>) -> (u128, u128) {
    tips.sort_unstable();
    let median = match tips.len() {
        0 => 0,
        n if n % 2 == 1 => tips[n / 2],
        n => tips[n / 2 - 1] / 2 + tips[n / 2] / 2,
    };
    let tip = median.max(MIN_TIP_WEI);
    (tip, next_base.saturating_mul(2).saturating_add(tip))
}
