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
 * A publisher call's `to` and calldata. `to` is always the config address
 * of the kind's contract on that chain (None when it is not deployed), and
 * the selector is settle_proto's, pinned there against its signature, so
 * what the wallet signs is what INTENT checks.
 */

use alloc::vec::Vec;

use nonos_nox_config::{address, Contract};
use nonos_settle_proto::{https_target, KINDS, MAX_URL};

use super::abi::{addr, call, uint};
use super::call::Publisher;
use super::decode_words::SPENDERS;

pub(super) fn selector(kind: u8) -> Option<u32> {
    KINDS.iter().find(|k| k.id == kind).map(|k| k.calls[0].1)
}

pub fn to(p: &Publisher, chain_id: u64) -> Option<[u8; 20]> {
    let k = KINDS.iter().find(|k| k.id == p.kind())?;
    address(chain_id, k.contract).ok()
}

pub fn calldata(p: &Publisher, chain_id: u64) -> Option<Vec<u8>> {
    encode(p, |c| address(chain_id, c).ok())
}

/// The calldata with `at` naming each contract's address.
pub fn encode(p: &Publisher, at: impl Fn(Contract) -> Option<[u8; 20]>) -> Option<Vec<u8>> {
    use Publisher::*;
    let sel = selector(p.kind())?;
    Some(match p {
        Approve { spender, amount } if SPENDERS.contains(spender) && *amount > 0 => {
            call(sel, &[addr(&at(*spender)?), uint(*amount)], None)
        }
        Approve { .. } => return None,
        Bond { publisher_key, amount } if *amount > 0 => {
            call(sel, &[*publisher_key, uint(*amount)], None)
        }
        Bond { .. } => return None,
        SetDeliveryAgent { bond_id, agent } => call(sel, &[*bond_id, addr(agent)], None),
        RequestUnbond { bond_id } | WithdrawBond { bond_id } => call(sel, &[*bond_id], None),
        Publish { publisher_key, app_id_hash, release_id, release_hash, bond_id, uri } => {
            if uri.len() > MAX_URL || https_target(uri).is_none() {
                return None;
            }
            let heads = [*publisher_key, *app_id_hash, *release_id, *release_hash, *bond_id];
            call(sel, &heads, Some(uri))
        }
        WithdrawRelease { listing_id, release_id } => call(sel, &[*listing_id, *release_id], None),
        Anchor { anchor, kind, listing_id } => {
            call(sel, &[*anchor, uint(*kind as u128), *listing_id], None)
        }
        WithdrawBondCredit { to } => call(sel, &[addr(to)], None),
    })
}
