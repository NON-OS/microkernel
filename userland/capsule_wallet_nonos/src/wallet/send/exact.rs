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

//! An amount in base units, written out with every digit it has: a review
//! that cut it to six places showed 0.0000005 ETH as 0, and a fee "at most"
//! cut down is less than the most. Trailing zeros after the point are left
//! off, and a whole amount has no point. Pure, so wallet_proofs holds it.

use alloc::string::String;

/// `v` base units at `dp` decimals, exactly.
pub fn exact_text(v: u128, dp: u32) -> String {
    let Some(one) = 10u128.checked_pow(dp) else { return alloc::format!("{v}") };
    let (whole, frac) = (v / one, v % one);
    if frac == 0 {
        return alloc::format!("{whole}");
    }
    let digits = alloc::format!("{frac:0width$}", width = dp as usize);
    alloc::format!("{whole}.{}", digits.trim_end_matches('0'))
}
