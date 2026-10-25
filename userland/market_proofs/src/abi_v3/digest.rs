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

use nonos_marketplace_abi::binding_digest_at;

use super::binding::BOND_AT;
use super::hosts::{keccak, Host};

#[test]
fn the_digest_is_eip_712_built_by_hand() {
    let (key, listing, bond) = ([1u8; 32], [2u8; 32], [3u8; 32]);
    let ty = keccak(
        b"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)",
    );
    let mut chain = [0u8; 32];
    chain[31] = 1;
    let mut at = [0u8; 32];
    at[12..].copy_from_slice(&BOND_AT);
    let sep = keccak(&[ty, keccak(b"NONOS Bond"), keccak(b"1"), chain, at].concat());
    let st = keccak(b"PublisherBinding(bytes32 publisherKey,bytes32 listingId,bytes32 bondId)");
    let sh = keccak(&[st, key, listing, bond].concat());
    let want = keccak(&[&[0x19u8, 0x01][..], &sep, &sh].concat());
    assert_eq!(binding_digest_at(&Host, 1, BOND_AT, [&key, &listing, &bond]), want);
}
