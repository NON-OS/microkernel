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
 * A publisher transaction as it is signed. The calldata is the wallet's
 * own encode, built once when the send is made and never taken from
 * settle (D5): settle's quote gives only `to`, the nonce and the gas
 * terms, held first to the intent by quote_refusal. The review (page.rs)
 * is read back from this same calldata.
 */

use alloc::vec::Vec;

use nonos_settle_proto::Quote;

use super::call::Publisher;
use super::encode::{calldata, to};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PublisherSend {
    pub kind: u8,
    pub chain_id: u64,
    pub from: [u8; 20],
    pub calldata: Vec<u8>,
}

impl PublisherSend {
    /// None while the contract called, or one it names, is not on
    /// `chain_id`.
    pub fn new(p: &Publisher, chain_id: u64, from: [u8; 20]) -> Option<Self> {
        to(p, chain_id)?;
        Some(Self { kind: p.kind(), chain_id, from, calldata: calldata(p, chain_id)? })
    }
}

/// The EIP-1559 fields signed: no ETH ever goes with a publisher call.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Signable {
    pub to: [u8; 20],
    pub value: u128,
    pub data: Vec<u8>,
    pub nonce: u64,
    pub gas: u64,
    pub max_priority: u128,
    pub max_fee: u128,
}

pub fn signable(send: &PublisherSend, q: &Quote) -> Signable {
    Signable {
        to: q.to,
        value: 0,
        data: send.calldata.clone(),
        nonce: q.nonce,
        gas: q.gas_limit,
        max_priority: q.max_priority,
        max_fee: q.max_fee,
    }
}
