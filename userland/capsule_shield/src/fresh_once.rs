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

//! OP_FRESH_ADDRESS mode 1, "once": the address for one
//! escrow order's refundTo or one channel's reclaimTo. Answered at once from
//! the addresses the worker read after the last job, past every account and
//! every index handed out, and kept as handed before it is answered.

use shield_wire::Values;

use crate::ops::{seen_ahead, Answer};

const NOT_OPEN: &str = "Open the shield from the wallet first.";
const USED_UP: &str =
    "The fresh addresses this unlock made are all handed out; lock and unlock the shield.";

pub fn fresh_once() -> Answer {
    let (wallet, count, ahead) = seen_ahead().ok_or_else(|| String::from(NOT_OPEN))?;
    let mut v = Values::new();
    if ahead.is_empty() {
        // A wallet from a private key has no other address: the same as mode 0.
        return Ok(v);
    }
    match wallet.fresh_once(count, &ahead).map_err(|e| e.to_string())? {
        Some(address) => {
            v.put("address", &address);
            Ok(v)
        }
        None => Err(String::from(USED_UP)),
    }
}
