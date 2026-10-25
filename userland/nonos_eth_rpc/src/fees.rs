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

//! A transaction's tip and fee cap from parse_fee_history's answer: the
//! median of the recent median tips, at least MIN_TIP_WEI, and a cap of twice
//! the next base fee plus the tip.

use alloc::vec::Vec;

/// The least tip offered, 0.01 gwei: a block builder may pass over none at all.
pub const MIN_TIP_WEI: u128 = 10_000_000;

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
