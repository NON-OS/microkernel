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

/* The intent tests' shared fixture: a host keccak, and contract addresses
 * that exist only here, one per contract and none alike. */

use nonos_marketplace_abi::Keccak;
use nonos_nox_config::Contract;
use nonos_settle_proto::Quote;
use sha3::{Digest, Keccak256};

pub(crate) struct K;
impl Keccak for K {
    fn keccak256(&self, d: &[u8]) -> [u8; 32] {
        Keccak256::digest(d).into()
    }
}

pub(crate) fn at(c: Contract) -> Option<[u8; 20]> {
    let mut a = [0xA5; 20];
    a[19] = c as u8;
    Some(a)
}

/// A uint256 word.
pub(crate) fn n(v: u128) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[16..].copy_from_slice(&v.to_be_bytes());
    w
}

pub(crate) fn nox(v: u128) -> String {
    format!("{v}")
}

/// A quote for `kind` at that kind's fixture contract.
pub(crate) fn quote(kind: u8) -> Quote {
    let c = nonos_settle_proto::KINDS.iter().find(|k| k.id == kind).unwrap().contract;
    Quote {
        kind,
        to: at(c).unwrap(),
        nonce: 4,
        gas_limit: 60_000,
        max_fee: 2_000_000_000,
        max_priority: 100_000_000,
    }
}
