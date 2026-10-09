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

//! The anonymity defaults a spend is held to before any proving time is
//! spent: it goes out through a submitter, and every amount anyone else can
//! see is one of the standard sizes, 1, 2 or 5 times a power of ten.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use stark_proofs::crypto::stark::air::{Poseidon, RATE};
use stark_proofs::crypto::stark::field::Fp;
use stark_proofs::shield::key::derive;
use stark_proofs::shield::live_seed::seed_notes;
use stark_proofs::shield::note::POOL_LOG_ROUNDS;

use crate::json::{try_unpack_digest, Json};

/// The smallest standard note per asset, in note units: ETH (asset 0)
/// counts wei, so 0.001 ETH is 10^15; NOX (asset 1) counts 10^9 base units
/// per note unit, so 0.001 NOX is 10^6.
pub const ASSET_UNITS: [(u64, u64); 2] = [(0, 1_000_000_000_000_000), (1, 1_000_000)];

pub fn unit_for(asset_id: u64) -> Option<u64> {
    ASSET_UNITS.iter().find(|(a, _)| *a == asset_id).map(|(_, u)| *u)
}

pub fn is_standard(v: u64, unit: u64) -> bool {
    if unit == 0 || v == 0 || v % unit != 0 {
        return false;
    }
    let mut m = v / unit;
    while m % 10 == 0 {
        m /= 10;
    }
    matches!(m, 1 | 2 | 5)
}

fn flag(req: &Json<'_>, name: &str) -> bool {
    req.has(name) && req.try_field(name).map(|v| v.starts_with("true")).unwrap_or(false)
}

/// The spend keys of the seed notes: change to one of them is the spender's
/// own and exempt from the standard sizes.
pub fn own_keys(seed: &str) -> Vec<[Fp; RATE]> {
    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    match seed_notes(seed) {
        Ok((sks, _)) => sks.iter().map(|sk| derive(&h, *sk).spend_pk).collect(),
        Err(_) => Vec::new(),
    }
}

pub fn check(request: &str, own: &[[Fp; RATE]]) -> Result<Vec<&'static str>, String> {
    let req = Json(request);
    let mut weakened = Vec::new();
    if flag(&req, "self_submit") {
        weakened.push("self_submit");
    } else if req.try_u64("fee")? == 0 {
        return Err("a spend goes out through a submitter by default: set a nonzero fee and its \
             fee_recipient, or \"self_submit\": true to send it from your own address"
            .to_string());
    }
    if flag(&req, "any_amount") {
        weakened.push("any_amount");
        return Ok(weakened);
    }
    let unit = if req.has("unit") {
        req.try_u64("unit")?
    } else {
        let asset = if req.has("asset_id") { req.try_u64("asset_id")? } else { 0 };
        unit_for(asset).ok_or_else(|| {
            format!("asset {asset} has no standard unit in the table; state it with \"unit\"")
        })?
    };
    let public = req.try_u64("public_amount")?;
    if public != 0 && !is_standard(public, unit) {
        return Err(format!(
            "public amount {public} is not a standard size of {unit}; split it, or set \"any_amount\": true"
        ));
    }
    let values = req.try_u64s("output_values")?;
    let foreign: Vec<bool> = if req.has("output_spend_pk") {
        let mut f = Vec::new();
        for w in req.try_strings("output_spend_pk")? {
            f.push(w != "self" && !own.contains(&try_unpack_digest(w)?));
        }
        f
    } else {
        vec![false; values.len()]
    };
    for (v, f) in values.iter().zip(foreign) {
        if f && !is_standard(*v, unit) {
            return Err(format!(
                "the payee's note of {v} is not a standard size of {unit}; split the payment, or set \"any_amount\": true"
            ));
        }
    }
    Ok(weakened)
}
