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
//! SHIELD_FROM_FRESH (op 18, v1.4.56) on the worker: NOX found at a fresh
//! address, moved into the Shield from that address. shield_core checks the
//! address was handed out, splits the NOX into standard deposits, approves
//! the pool for exactly their sum, and signs with that index's key alone,
//! derived for the sends and dropped zeroed.

use shield_wire::{unhex, Values};

use crate::ops::{wallet, Answer};

const UNREADABLE: &str = "That request to shield is not readable.";

/// "review", the address, the NOX there in wei.
pub fn review(fields: &[&str]) -> Answer {
    let [_, address, amount] = fields else { return Err(String::from(UNREADABLE)) };
    let address = unhex(address).and_then(|b| <[u8; 20]>::try_from(b).ok());
    let address = address.ok_or_else(|| String::from(UNREADABLE))?;
    let amount: u128 = amount.parse().map_err(|_| String::from(UNREADABLE))?;
    let r = wallet()?.review_shield_from_fresh(address, amount).map_err(|e| e.to_string())?;
    let s = &r.split;
    let mut v = Values::new();
    v.put("id", &s.id.to_string())
        .put("approval", if s.approval { "1" } else { "0" })
        .put("pieces", &s.pieces.len().to_string())
        .put("amount", &s.amount)
        .put("pool_fee", &s.pool_fee)
        .put("shielded", &s.shielded)
        .put("max_network_fee", &s.max_network_fee)
        .put("valid_for", &s.valid_for_seconds.to_string())
        .put("remainder", &r.remainder.to_string())
        .put("short_eth", &r.short_eth.to_string());
    if let Some(why) = &s.refusal {
        v.put("refusal", why);
    }
    Ok(v)
}

/// "confirm" and the review's id: each transaction sent, in order.
pub fn confirm(fields: &[&str]) -> Answer {
    let [_, id] = fields else { return Err(String::from("Name the review.")) };
    let id: u64 = id.parse().map_err(|_| String::from("That review is not a number."))?;
    let sent = wallet()?.confirm_shield_from_fresh(id).map_err(|e| e.to_string())?;
    let mut v = Values::new();
    for s in &sent.sent {
        v.put("tx", &s.hash);
    }
    if let Some(why) = &sent.stopped {
        v.put("stopped", why);
    }
    Ok(v)
}
