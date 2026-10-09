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

/*
 * The standard amounts, 1, 2 or 5 times a power of ten: 0.01 to 10 ETH and
 * 1,000 to 5,000,000 NOX. Both tokens have 18 decimals, so each size is also
 * its amount in base units, which is what the pool compares.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::wallet::state::shield_ui::ASSET_NOX;

const UNIT: u128 = 1_000_000_000_000_000_000;

/* Every size between `lo` and `hi`, both counted in `step`s of the token. */
fn ladder(lo: u128, hi: u128) -> Vec<u128> {
    let mut out = Vec::new();
    let mut ten = 1u128;
    while ten <= hi {
        for m in [1u128, 2, 5] {
            let v = m * ten;
            if v >= lo && v <= hi {
                out.push(v);
            }
        }
        ten *= 10;
    }
    out
}

fn grouped(mut v: u128) -> String {
    let mut parts = Vec::new();
    while v >= 1000 {
        parts.push(format!("{:03}", v % 1000));
        v /= 1000;
    }
    parts.push(format!("{v}"));
    parts.reverse();
    parts.join(",")
}

fn hundredths(c: u128) -> String {
    match c % 100 {
        0 => format!("{}", c / 100),
        f if f % 10 == 0 => format!("{}.{}", c / 100, f / 10),
        f => format!("{}.{:02}", c / 100, f),
    }
}

/* Each size as it is shown and as base units. */
pub fn sizes(asset: u8) -> Vec<(String, u128)> {
    if asset == ASSET_NOX {
        return ladder(1_000, 5_000_000).into_iter().map(|n| (grouped(n), n * UNIT)).collect();
    }
    ladder(1, 1_000).into_iter().map(|c| (hundredths(c), c * (UNIT / 100))).collect()
}
