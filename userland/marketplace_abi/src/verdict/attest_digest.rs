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

//! The EIP-712 digest of a ReleaseAttest (plan/23) under the bond contract's
//! domain, and the six words a release attests to.

use nonos_nox_config::{domain_words_at, BOND, RELEASE_ATTEST};

use crate::ids::{listing_id, release_id, Keccak};
use crate::types::{CapsuleRelease, Commerce, MarketplaceEntry, SettleMode};

/// listingId, releaseId, bondId, chunkRoot, contentLen, packageHash. An
/// escrow release attests to its chunk tree and length with packageHash
/// zero; any other attests to its package hash with chunkRoot and
/// contentLen zero. So the attest pins exactly what the installer pins.
pub fn attest_words(
    k: &impl Keccak,
    entry: &MarketplaceEntry,
    c: &Commerce,
    rel: &CapsuleRelease,
) -> [[u8; 32]; 6] {
    let listing = listing_id(k, &entry.publisher_pubkey, &entry.listing_id);
    let release = release_id(k, &listing, &rel.release_id);
    let mut words = [listing, release, c.bond_id, [0; 32], [0; 32], [0; 32]];
    match (c.settle_mode, rel.ext.as_ref()) {
        (SettleMode::Escrow, Some(x)) => {
            words[3] = x.chunk_root;
            words[4][24..].copy_from_slice(&x.content_len.to_be_bytes());
        }
        _ => words[5] = rel.package_hash,
    }
    words
}

pub fn attest_digest_at(
    k: &impl Keccak,
    chain_id: u64,
    bond_contract: [u8; 20],
    words: &[[u8; 32]; 6],
) -> [u8; 32] {
    let separator = k.keccak256(&domain_words_at(&BOND, chain_id, bond_contract));
    let mut pre = [0u8; 32 * 7];
    pre[..32].copy_from_slice(&RELEASE_ATTEST.typehash);
    for (i, w) in words.iter().enumerate() {
        pre[32 * (i + 1)..32 * (i + 2)].copy_from_slice(w);
    }
    let struct_hash = k.keccak256(&pre);
    let mut msg = [0u8; 66];
    msg[..2].copy_from_slice(&[0x19, 0x01]);
    msg[2..34].copy_from_slice(&separator);
    msg[34..].copy_from_slice(&struct_hash);
    k.keccak256(&msg)
}
