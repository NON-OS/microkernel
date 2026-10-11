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

//! The view table. Each selector is pinned by a test against its signature.

use nonos_nox_config::Contract;

use crate::view::{Arg, View};

use Arg::{Address as A, Word as W, U64, U8};
use Contract::{Escrow as E, ListingRegistry as R, NoxToken as N, ProofAnchor as P};
use Contract::{PublisherBond as B, Settlement as S, ShieldPool as X};

const fn v(
    id: u16,
    contract: Contract,
    signature: &'static str,
    sel: u32,
    args: &'static [Arg],
) -> View {
    View { id, contract, signature, selector: sel.to_be_bytes(), args }
}

pub const VIEWS: [View; 34] = [
    v(1, N, "balanceOf(address)", 0x70a08231, &[A]),
    v(2, N, "allowance(address,address)", 0xdd62ed3e, &[A, A]),
    v(3, N, "feeExempt(address)", 0x398daa85, &[A]),
    v(4, N, "isBlacklisted(address)", 0xfe575a87, &[A]),
    v(10, S, "deposited(bytes32)", 0x1f36edcd, &[W]),
    v(11, S, "settled(bytes32)", 0xd945af1d, &[W]),
    v(12, S, "expiryOf(bytes32)", 0x40711fb0, &[W]),
    v(13, S, "closed(bytes32)", 0x3fb8c56d, &[W]),
    v(14, S, "openNonce(address)", 0xa857df04, &[A]),
    v(15, S, "credit(address)", 0xd5d44d80, &[A]),
    v(16, S, "GRACE()", 0x137d29d9, &[]),
    v(17, S, "feeBps()", 0x24a9d853, &[]),
    v(18, S, "eip712Domain()", 0x84b0196e, &[]),
    v(20, E, "orderOf(bytes32)", 0xd5a014e5, &[W]),
    v(21, E, "depositAddress(bytes32)", 0x31eecaf4, &[W]),
    v(22, E, "credit(address)", 0xd5d44d80, &[A]),
    v(23, E, "MAX_DELIVERY()", 0x503865a9, &[]),
    v(24, E, "DISPUTE_WINDOW()", 0xf585dc57, &[]),
    v(25, E, "FORWARDER_INITCODE_HASH()", 0x85cf6351, &[]),
    v(26, E, "feeBps()", 0x24a9d853, &[]),
    v(27, E, "eip712Domain()", 0x84b0196e, &[]),
    v(30, B, "bondOf(bytes32)", 0x0fb585ba, &[W]),
    v(31, B, "isActive(bytes32)", 0x5c36901c, &[W]),
    v(32, B, "deliveryAgentOf(bytes32)", 0xeeaefa92, &[W]),
    v(33, B, "MIN_BOND()", 0xe0de0fee, &[]),
    v(34, B, "UNBOND_DELAY()", 0x5188abbb, &[]),
    v(35, B, "credit(address)", 0xd5d44d80, &[A]),
    v(36, B, "escrow()", 0xe2fdcc17, &[]),
    v(37, B, "eip712Domain()", 0x84b0196e, &[]),
    v(40, R, "releaseOf(bytes32,bytes32)", 0xfb9006bf, &[W, W]),
    v(41, R, "eip712Domain()", 0x84b0196e, &[]),
    v(50, P, "anchoredAt(bytes32,uint8,bytes32)", 0xc4aea52d, &[W, U8, W]),
    v(51, P, "eip712Domain()", 0x84b0196e, &[]),
    v(60, X, "claimable(uint64,address)", 0x88d8b2a7, &[U64, A]),
];
