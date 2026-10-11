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

//! INTENT: check that the calldata's selector belongs to its kind, and quote
//! it: `to` is the kind's config contract, its NOX flags read again now; an
//! approve's spender must be the config Settlement, Escrow or Bond, read the
//! same way; a ShieldClaim must pay its sender. Then the sender's next nonce, the node's estimate (padded by a
//! fifth) and fee suggestions, in one batch. The calldata is the key
//! holder's; settle returns none and changes none.

use alloc::vec::Vec;

use nonos_eth_rpc::{object_for_id, parse_u64, request_batch, request_estimate_gas};
use nonos_eth_rpc::{request_fee_history, request_nonce};
use nonos_nox_config::{address, Contract};
use nonos_settle_proto::{approve_spender, claim_ok, kind_of, padded, Intent, Quote, EINVAL};

use super::gas::{fees_in, nonce_in};
use crate::settle::Settle;

const SPENDERS: [Contract; 3] = [Contract::Settlement, Contract::Escrow, Contract::PublisherBond];

pub fn intent(s: &mut Settle, body: &[u8]) -> Result<Vec<u8>, i32> {
    let i = Intent::parse(body).ok_or(EINVAL)?;
    let kind = kind_of(i.kind, &i.calldata).ok_or(EINVAL)?;
    if kind.contract == Contract::ShieldPool && !claim_ok(&i.calldata, &i.from) {
        return Err(EINVAL);
    }
    let to = s.live.fresh(&mut s.rpc, i.chain_id, kind.contract)?;
    if kind.contract == Contract::NoxToken {
        let spender = approve_spender(&i.calldata).ok_or(EINVAL)?;
        let named = SPENDERS.iter().find(|c| address(i.chain_id, **c) == Ok(spender));
        s.live.fresh(&mut s.rpc, i.chain_id, *named.ok_or(EINVAL)?)?;
    }
    let parts = [
        request_nonce(&i.from, 1),
        request_estimate_gas(&i.from, &to, i.value, &i.calldata, 2),
        request_fee_history(3),
    ];
    let refs: Vec<&[u8]> = parts.iter().map(Vec::as_slice).collect();
    let resp = s.rpc.call(i.chain_id, &request_batch(&refs))?;
    /* An estimate the node refuses is a call that would revert. */
    let estimate = object_for_id(&resp, 2).and_then(parse_u64).ok_or(EINVAL)?;
    let (tip, cap) = fees_in(&resp, 3)?;
    let quote = Quote {
        kind: kind.id,
        to,
        nonce: nonce_in(&resp, 1)?,
        gas_limit: padded(estimate).ok_or(EINVAL)?,
        max_fee: cap,
        max_priority: tip,
    };
    Ok(quote.encode().to_vec())
}
