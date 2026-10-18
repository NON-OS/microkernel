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

//! GAS: chain_id u64 | address -> its balance, its next nonce counting what
//! waits, and fee suggestions from eth_feeHistory. Suggestions only: every
//! signer bounds them (nonos_nox_config's caps).

use alloc::vec::Vec;

use nonos_eth_rpc::{fees, object_for_id, parse_fee_history, parse_quantity32, parse_u64};
use nonos_eth_rpc::{request_balance, request_batch, request_fee_history, request_nonce};
use nonos_settle_proto::{Gas, EINVAL, EPROTO};

use crate::chain::word_u128;
use crate::settle::Settle;

/// The next nonce of the answer's object `id`.
pub(super) fn nonce_in(resp: &[u8], id: u64) -> Result<u64, i32> {
    object_for_id(resp, id).and_then(parse_u64).ok_or(EPROTO)
}

/// (tip, cap) per gas from the answer's fee history object `id`.
pub(super) fn fees_in(resp: &[u8], id: u64) -> Result<(u128, u128), i32> {
    let (next, tips) = object_for_id(resp, id).and_then(parse_fee_history).ok_or(EPROTO)?;
    Ok(fees(next, tips))
}

pub fn gas(s: &mut Settle, body: &[u8]) -> Result<Vec<u8>, i32> {
    let b: &[u8; 28] = body.try_into().map_err(|_| EINVAL)?;
    let chain_id = u64::from_le_bytes(b[..8].try_into().map_err(|_| EINVAL)?);
    let addr: [u8; 20] = b[8..].try_into().map_err(|_| EINVAL)?;
    Ok(quote_gas(s, chain_id, &addr)?.encode().to_vec())
}

/// `addr`'s balance, next nonce and fee suggestions, in one batch.
pub(crate) fn quote_gas(s: &mut Settle, chain_id: u64, addr: &[u8; 20]) -> Result<Gas, i32> {
    let parts = [request_balance(addr, 1), request_nonce(addr, 2), request_fee_history(3)];
    let refs: Vec<&[u8]> = parts.iter().map(Vec::as_slice).collect();
    let resp = s.rpc.call(chain_id, &request_batch(&refs))?;
    let balance = object_for_id(&resp, 1).and_then(parse_quantity32).ok_or(EPROTO)?;
    let (tip, cap) = fees_in(&resp, 3)?;
    Ok(Gas {
        balance: word_u128(&balance).ok_or(EPROTO)?,
        tx_nonce: nonce_in(&resp, 2)?,
        max_fee_per_gas: cap,
        max_priority_fee_per_gas: tip,
    })
}
