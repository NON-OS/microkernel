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

//! INTENT's kinds, numbered from 1. Each names the contract that
//! is its `to` and the calls whose selector belongs to it. Each selector is
//! pinned by a test against its signature. 17, 18 and 19 are one call,
//! withdrawCredit(address), on Settlement, the Escrow and the Bond (v1.4.6).
//! 20 is the Shield pool's claim(assetId, to) (v1.4.17).

use nonos_nox_config::Contract;

use Contract::{Escrow as E, ListingRegistry as R, NoxToken as N, ProofAnchor as P};
use Contract::{PublisherBond as B, Settlement as S, ShieldPool as X};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Kind {
    pub id: u8,
    pub contract: Contract,
    pub calls: &'static [(&'static str, u32)],
}

const fn k(id: u8, contract: Contract, calls: &'static [(&'static str, u32)]) -> Kind {
    Kind { id, contract, calls }
}

/// A signature's "T" is OrderTerms' tuple, and its "V" is
/// Voucher's tuple, so each table line stays one line.
pub const ORDER_TERMS_TUPLE: &str = "(bytes32,bytes32,bytes32,address,address,uint256,uint64,\
uint64,bytes32,uint64,bytes32,bytes32,bytes32,bytes32)";
pub const VOUCHER_TUPLE: &str =
    "(bytes32,address,address,bytes32,bytes32,uint256,uint64,uint64,bytes32)";

pub const KINDS: [Kind; 20] = [
    k(1, N, &[("approve(address,uint256)", 0x095ea7b3)]),
    k(2, B, &[("bond(bytes32,uint256)", 0x51f93215)]),
    k(3, B, &[("setDeliveryAgent(bytes32,address)", 0xc6a70f23)]),
    k(4, B, &[("requestUnbond(bytes32)", 0x977d178f)]),
    k(5, B, &[("withdraw(bytes32)", 0x8e19899e)]),
    k(6, R, &[("publish(bytes32,bytes32,bytes32,bytes32,bytes32,string)", 0x8a6b25dd)]),
    k(7, R, &[("withdraw(bytes32,bytes32)", 0x63615149)]),
    k(8, P, &[("anchor(bytes32,uint8,bytes32)", 0x5c4993f7)]),
    k(9, E, &[("fund(T)", 0xb18d590d), ("fundFrom(T,uint256)", 0xaba72c43)]),
    k(10, E, &[("dispute(T,uint64,bytes,bytes32,bytes32[],bytes32[])", 0xd84d6ef9)]),
    k(11, E, &[("refund(T,address,bytes32)", 0xa3814a14)]),
    k(12, E, &[("release(T)", 0x2fc866f5)]),
    k(13, S, &[("open(address,bytes32,uint256,uint64,address)", 0x631969a5)]),
    k(14, S, &[("topUp(bytes32,uint256)", 0xb67644b9)]),
    k(15, S, &[("close(bytes32)", 0x39c79e0c)]),
    k(16, S, &[("settle(V[],bytes[])", 0x13508ec8)]),
    k(17, S, &[("withdrawCredit(address)", 0x6c8cc9d9)]),
    k(18, E, &[("withdrawCredit(address)", 0x6c8cc9d9)]),
    k(19, B, &[("withdrawCredit(address)", 0x6c8cc9d9)]),
    k(20, X, &[("claim(uint64,address)", 0x881a1ce0)]),
];
