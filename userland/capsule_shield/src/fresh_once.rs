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

/// Mode 2: each address handed out as `handed` = index|0x address, read
/// locally from the shield's own file; the wallet reads none of them over
/// the network unless money is known due there (v1.4.56).
pub fn fresh_handed() -> Answer {
    let (wallet, _, _) = seen_ahead().ok_or_else(|| String::from(NOT_OPEN))?;
    let mut v = Values::new();
    for (i, a) in wallet.fresh_handed() {
        v.put("handed", &format!("{i}|0x{a}"));
    }
    Ok(v)
}

/// Mode 3, a job: the restore scan's batch of addresses from index `from`,
/// derived only when the person pressed scan.
pub fn fresh_batch(fields: &[&str]) -> Answer {
    let from = fields.get(1).and_then(|f| f.parse::<u32>().ok());
    let from = from.ok_or_else(|| String::from("Which index to scan from is not readable."))?;
    let (wallet, count, _) = seen_ahead().ok_or_else(|| String::from(NOT_OPEN))?;
    // 0 is "past the accounts": a restore starts where the accounts end.
    let from = if from == 0 { count } else { from };
    let mut v = Values::new();
    for (i, a) in wallet.fresh_batch(from).map_err(|e| e.to_string())? {
        let hex: String = a.iter().map(|b| format!("{b:02x}")).collect();
        v.put("handed", &format!("{i}|0x{hex}"));
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
