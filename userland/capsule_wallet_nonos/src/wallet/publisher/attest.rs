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
 * Entry 3, ReleaseAttest, for one release of an entry document: the attest
 * the wallet logs and signs, taken from marketplace_abi's own words for that
 * release (so what the market checks and what is signed are one value), and
 * the rows the screen shows: listingId, release name, chunkRoot, contentLen
 * and packageHash.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_marketplace_abi::{attest_words as release_words, Keccak, MarketplaceEntry};

use super::super::attest::Attest;
use super::super::hex_id::{hex, id};
use super::binding::Refused;

pub fn attest_of(
    k: &impl Keccak,
    e: &MarketplaceEntry,
    release: usize,
    chain_id: u64,
) -> Result<Attest, Refused> {
    let c = e.commerce.as_ref().ok_or(Refused::NotSold)?;
    let rel = e.releases.get(release).ok_or(Refused::NoRelease)?;
    let w = release_words(k, e, c, rel);
    let mut len = [0u8; 8];
    len.copy_from_slice(&w[4][24..]);
    if w[4][..24].iter().any(|b| *b != 0) {
        return Err(Refused::Shape);
    }
    Ok(Attest {
        chain_id,
        listing_id: w[0],
        release_id: w[1],
        bond_id: w[2],
        chunk_root: w[3],
        content_len: u64::from_be_bytes(len),
        package_hash: w[5],
    })
}

pub fn attest_rows(
    e: &MarketplaceEntry,
    release: usize,
    a: &Attest,
) -> Vec<(&'static str, String)> {
    let name = e.releases.get(release).map(|r| r.release_id.clone()).unwrap_or_default();
    Vec::from([
        ("listing", id(&a.listing_id)),
        ("release", name),
        ("chunk root", hex(&a.chunk_root)),
        ("content length", format!("{} bytes", a.content_len)),
        ("package hash", hex(&a.package_hash)),
    ])
}
