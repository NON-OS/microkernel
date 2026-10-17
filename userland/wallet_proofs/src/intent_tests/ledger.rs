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

/* The send-ledger tests' fixture: one publisher send from this account on
 * chain 1, ledger entries by account, nonce and hash tag, and GAS. */

use nonos_settle_proto::{Gas, Quote, KINDS};

use super::decode::built;
use super::fixture::at;
use super::kinds::every;
use crate::intent::{held, Publisher, PublisherSend};
use crate::sent::{Fate, Sent};

pub(super) const ME: [u8; 20] = [9; 20];

pub(super) fn send() -> PublisherSend {
    let p = every().remove(0);
    PublisherSend { kind: p.kind(), chain_id: 1, from: ME, calldata: built(&p) }
}

pub(super) fn entry(from: [u8; 20], nonce: u64, tag: u8) -> Sent {
    let (hash, eth_cost, token, fate) = ([tag; 32], 1, None, Fate::Waiting);
    Sent { chain: 1, from, nonce, hash, at_ms: 0, eth_cost, token, fate, misses: 0, unknown: false }
}

pub(super) fn gas(tx_nonce: u64) -> Gas {
    Gas { balance: u128::MAX, tx_nonce, max_fee_per_gas: 1, max_priority_fee_per_gas: 1 }
}

pub(super) fn bounds(p: &Publisher, q: &Quote) -> Option<&'static str> {
    held(p, KINDS.iter().find(|k| k.id == p.kind()).and_then(|k| at(k.contract)), q)
}
