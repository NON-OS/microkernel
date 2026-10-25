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

//! What an open says, and where the pool is.

use super::*;

/* Said by every shield step asked off the one network the pool runs on. */
pub const ELSEWHERE: &str = "The shield pool is not deployed on Ethereum mainnet. It runs on \
     Sepolia: switch there to use it.";

/* An open is a few file writes and under a second of key derivation, even
 * on an emulated machine. Past this it is stuck, not slow. */
pub const OPEN_STUCK_S: u32 = 60;

/* What an open's wait says after its elapsed time: where the address will
 * appear, or, past `OPEN_STUCK_S`, that the service is stuck at the step
 * named. Any other job says nothing more. */
pub fn opening_said(op: u16, elapsed_s: u32) -> &'static str {
    if !matches!(op, OP_OPEN_WORDS | OP_OPEN_KEY) {
        return "";
    }
    if elapsed_s < OPEN_STUCK_S {
        " The private address appears here when it is done, in a few seconds."
    } else {
        " This takes seconds, so the shield service is stuck at that step, not slow."
    }
}

/* Whether the picked network has the shield pool. */
pub fn here() -> bool {
    crate::wallet::chain::current().shield
}
