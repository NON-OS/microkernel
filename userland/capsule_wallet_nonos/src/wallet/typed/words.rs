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
 * The words of entries 2 and 3, under the wallet's rules (plan/31, plan/23).
 * Both name a bondId, and it must be keccak256(abi.encode(BOND_TAG, signer,
 * publisherKey)): this account's own bond for the key the person confirmed.
 * A ReleaseAttest is one of two shapes: escrow (chunkRoot and contentLen
 * set, packageHash zero) or a package (packageHash set, chunkRoot zero,
 * contentLen 0). Anything else is refused before the keyring is asked.
 */

use nonos_marketplace_abi::{bond_id, Keccak};

use super::super::attest::Attest;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Wrong {
    /// The bondId is not this account's bond for this publisher key.
    NotOurBond,
    /// Neither the escrow shape nor the package shape.
    Shape,
}

fn ours(k: &impl Keccak, signer: &[u8; 20], key: &[u8; 32], bond: &[u8; 32]) -> Result<(), Wrong> {
    (bond_id(k, signer, key) == *bond).then_some(()).ok_or(Wrong::NotOurBond)
}

pub fn binding_words(
    k: &impl Keccak,
    signer: &[u8; 20],
    publisher_key: &[u8; 32],
    listing_id: &[u8; 32],
    bond: &[u8; 32],
) -> Result<[[u8; 32]; 3], Wrong> {
    ours(k, signer, publisher_key, bond)?;
    Ok([*publisher_key, *listing_id, *bond])
}

pub fn attest_words(
    k: &impl Keccak,
    signer: &[u8; 20],
    publisher_key: &[u8; 32],
    a: &Attest,
) -> Result<[[u8; 32]; 6], Wrong> {
    ours(k, signer, publisher_key, &a.bond_id)?;
    let zero = [0u8; 32];
    let escrow = a.package_hash == zero && a.chunk_root != zero && a.content_len > 0;
    let package = a.package_hash != zero && a.chunk_root == zero && a.content_len == 0;
    if !(escrow || package) {
        return Err(Wrong::Shape);
    }
    let mut len = [0u8; 32];
    len[24..].copy_from_slice(&a.content_len.to_be_bytes());
    Ok([a.listing_id, a.release_id, a.bond_id, a.chunk_root, len, a.package_hash])
}
