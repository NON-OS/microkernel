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

//! Whether a config contract is the one the config describes: fee
//! exempt and not blacklisted on NOX (D11), and its eip712Domain, its time
//! constants and the pool's amountPolicy() (times.rs) and the bond's escrow()
//! all the config's. NOX and the policy, which holds no NOX, have none.

use nonos_nox_config::{address, domain_answer_matches, Contract, Domain};
use nonos_nox_config::{BOND, ESCROW, LISTING, SETTLEMENT};
use nonos_settle_proto::EPROTO;

use super::call_one::view_call;
use super::flags::flags;
use super::times::times_ok;
use super::words::addr_word;
use crate::net::Rpc;

fn domain(c: Contract) -> Option<(&'static Domain, u16)> {
    match c {
        Contract::Settlement => Some((&SETTLEMENT, 18)),
        Contract::Escrow => Some((&ESCROW, 27)),
        Contract::PublisherBond => Some((&BOND, 37)),
        Contract::ListingRegistry => Some((&LISTING, 41)),
        _ => None,
    }
}

/* A revert is a no; a host that could not be asked is the error. */
fn no_on_revert(r: Result<bool, i32>) -> Result<bool, i32> {
    match r {
        Err(EPROTO) => Ok(false),
        other => other,
    }
}

pub fn check(rpc: &mut Rpc, chain_id: u64, c: Contract, at: &[u8; 20]) -> Result<bool, i32> {
    if matches!(c, Contract::NoxToken | Contract::AmountPolicy) {
        return Ok(true);
    }
    if !flags(rpc, chain_id, at)? {
        return Ok(false);
    }
    if let Some((d, view)) = domain(c) {
        let matched = view_call(rpc, chain_id, at, view, &[])
            .map(|r| domain_answer_matches(d, chain_id, at, &r));
        if !no_on_revert(matched)? {
            return Ok(false);
        }
    }
    if !times_ok(rpc, chain_id, c, at)? {
        return Ok(false);
    }
    if c == Contract::PublisherBond {
        let Ok(escrow) = address(chain_id, Contract::Escrow) else {
            return Ok(false);
        };
        let said = view_call(rpc, chain_id, at, 36, &[]).map(|w| w == addr_word(&escrow));
        return no_on_revert(said);
    }
    Ok(true)
}
