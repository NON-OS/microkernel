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

/// How far past the accounts a machine with no handed list looks.
const SCAN: u32 = 20;
const NOT_OPEN: &str = "Open the shield from the wallet first.";
const USED_UP: &str =
    "The fresh addresses this unlock made are all handed out; lock and unlock the shield.";

/// Mode 2: each address handed out as `handed` = index|0x address. A
/// machine that kept none past the accounts, as after a restore, adds the
/// SCAN addresses past them, so a refund paid there is still found.
pub fn fresh_handed() -> Answer {
    let (wallet, count, ahead) = seen_ahead().ok_or_else(|| String::from(NOT_OPEN))?;
    let mut v = Values::new();
    let handed = wallet.fresh_handed();
    for (i, a) in &handed {
        v.put("handed", &format!("{i}|0x{a}"));
    }
    if !handed.iter().any(|(i, _)| *i > count) {
        for (i, a) in ahead.iter().filter(|(i, _)| *i < count.saturating_add(SCAN)) {
            let hex: String = a.iter().map(|b| format!("{b:02x}")).collect();
            v.put("handed", &format!("{i}|0x{hex}"));
        }
    }
    Ok(v)
}

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
