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
 * The review page of a publisher transaction, read back from the calldata
 * that is signed (send.rs) rather than from what was asked: the contract
 * and its address in full, the function as its signature, no ETH sent,
 * every argument decoded, then the quote's fee and gas terms.
 */

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use nonos_marketplace_abi::Keccak;
use nonos_settle_proto::{Quote, KINDS};

use super::decode::decode;
use super::quote::fee_rows;
use super::review::review_rows;
use super::review_bond::{At, Rows};
use super::text::placed;

/// The contract called at `to`, the function, and the value sent.
pub fn head_rows(kind: u8, to: &[u8; 20], k: &impl Keccak) -> Option<Rows> {
    let kd = KINDS.iter().find(|x| x.id == kind)?;
    Some(Vec::from([
        ("contract", placed(k, kd.contract, Some(*to))),
        ("function", kd.calls[0].0.to_string()),
        ("value", "0 ETH".to_string()),
    ]))
}

/// The whole page for `calldata` sent to the quote's `to`; None when the
/// calldata is not one this kind's encode writes, or the quote's kind
/// differs.
pub fn rows(
    kind: u8,
    calldata: &[u8],
    q: &Quote,
    k: &impl Keccak,
    at: At,
    nox: fn(u128) -> String,
    eth: fn(u128) -> String,
) -> Option<Rows> {
    let p = decode(kind, calldata, at).filter(|_| q.kind == kind)?;
    let mut out = head_rows(kind, &q.to, k)?;
    out.extend(review_rows(&p, k, at, nox));
    out.extend(fee_rows(q, eth));
    Some(out)
}
