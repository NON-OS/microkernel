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

//! SIGN_FROM_FRESH (op 17, plan/27, v1.4.51) on the worker: a refund or a
//! dispute signed by a fresh address this shield handed out. shield_core
//! checks the request, opens the vault again for the seed, derives that
//! index's key for this signature and drops it zeroed; the answer is the
//! signed transaction alone, for settle to broadcast.

use nox_shield_core::wallet::fresh::FreshSign;
use shield_wire::{hex, FreshAsk, Values};

use crate::ops::{wallet, Answer};

const UNREADABLE: &str = "That request to sign is not readable.";

pub fn sign_from_fresh(fields: &[&str]) -> Answer {
    let a = FreshAsk::parse(fields).ok_or_else(|| String::from(UNREADABLE))?;
    let r = FreshSign {
        address: a.address,
        chain_id: a.chain_id,
        nonce: a.nonce,
        gas: a.gas,
        max_fee: a.max_fee,
        max_priority: a.max_priority,
        to: a.to,
        value: a.value,
        data: a.data,
    };
    match wallet()?.sign_from_fresh(&r).map_err(|e| e.to_string())? {
        Ok(signed) => {
            let mut v = Values::new();
            v.put("raw", &hex(&signed.raw)).put("hash", &hex(&signed.hash));
            Ok(v)
        }
        Err(why) => Err(String::from(why.why())),
    }
}
