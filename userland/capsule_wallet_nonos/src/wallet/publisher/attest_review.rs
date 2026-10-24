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

//! What an attest's review shows before it is signed. A third party relies on
//! every attest, so it is reviewed and confirmed as the binding is: the whole
//! release as signed, the EIP-712 digest the keyring will sign, the account
//! that signs, and the Bond that holds it to its word. Pure, so the wallet
//! proofs run it on the host.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_marketplace_abi::{Keccak, MarketplaceEntry};
use nonos_nox_config::{domain_words_at, BOND, RELEASE_ATTEST};

use super::super::hex_id::hex;
use super::super::typed::check::digest;
use super::super::typed::{attest_words, struct_hash};
use super::attest::{attest_of, attest_rows};
use super::binding::Refused;

pub struct AttestReview {
    pub rows: Vec<(&'static str, String)>,
    pub digest: [u8; 32],
}

/// Release `release`'s attest as `signer` would sign it under `bond`.
pub fn attest_review(
    k: &impl Keccak,
    e: &MarketplaceEntry,
    release: usize,
    chain_id: u64,
    signer: &[u8; 20],
    bond: &[u8; 20],
) -> Result<AttestReview, Refused> {
    let a = attest_of(k, e, release, chain_id)?;
    let words =
        attest_words(k, signer, &e.publisher_pubkey, &a).map_err(|_| Refused::NotOurBond)?;
    let sh = struct_hash(k, &RELEASE_ATTEST, &words).ok_or(Refused::Shape)?;
    let d = digest(k, &k.keccak256(&domain_words_at(&BOND, chain_id, *bond)), &sh);
    let mut rows = Vec::from([("signs", String::from("a ReleaseAttest, EIP-712, NONOS Bond 1"))]);
    rows.extend(attest_rows(e, release, &a));
    rows.push(("signed by", hex(signer)));
    rows.push(("relied on by", format!("the Bond {} on chain {chain_id}", hex(bond))));
    rows.push(("digest", hex(&d)));
    Ok(AttestReview { rows, digest: d })
}
