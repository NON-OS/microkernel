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

//! How many decimal places each asset the send screen moves is counted in.
//!
//! Both are eighteen today, which is exactly why they are written down
//! separately: a single shared constant would read as a fact about tokens rather
//! than a coincidence between two of them, and the first asset that counts to six
//! would be scaled by a factor of a trillion by whoever reused it.

use crate::wallet::nox::NOX_DECIMALS;

pub const ETH_DECIMALS: u32 = 18;

/// The precision for whichever asset the send screen is set to move.
/// `send_token` is 0 for ETH and 1 for NOX.
pub fn send_decimals(send_token: u8) -> u32 {
    if send_token == 1 {
        NOX_DECIMALS
    } else {
        ETH_DECIMALS
    }
}
