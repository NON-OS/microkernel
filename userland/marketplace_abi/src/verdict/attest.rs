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

//! Whether the payee attested to this release's bytes. Advisory, like every
//! gate; two attests by one payee for one release that differ are what the
//! bond contract slashes.

use nonos_nox_config::{address, BOND};

use super::attest_digest::{attest_digest_at, attest_words};
use super::recover::Recover;
use crate::ids::Keccak;
use crate::types::{CapsuleRelease, MarketplaceEntry};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum AttestVerdict {
    Valid = 0,
    /// A schema 2 entry or release: there is no payee or no attest.
    NotV3 = 1,
    /// No bond contract on the token's chain to take the domain from.
    NoDomain = 2,
    /// The signature recovers to nobody, or to someone other than the payee.
    WrongSigner = 3,
}

pub fn attest_verdict(
    k: &impl Keccak,
    rec: &impl Recover,
    entry: &MarketplaceEntry,
    rel: &CapsuleRelease,
) -> AttestVerdict {
    match address(entry.token.chain_id, BOND.contract) {
        Ok(bond) => attest_verdict_at(k, rec, entry, rel, bond),
        Err(_) if entry.commerce.is_none() || rel.ext.is_none() => AttestVerdict::NotV3,
        Err(_) => AttestVerdict::NoDomain,
    }
}

pub fn attest_verdict_at(
    k: &impl Keccak,
    rec: &impl Recover,
    entry: &MarketplaceEntry,
    rel: &CapsuleRelease,
    bond_contract: [u8; 20],
) -> AttestVerdict {
    let (Some(c), Some(x)) = (entry.commerce.as_ref(), rel.ext.as_ref()) else {
        return AttestVerdict::NotV3;
    };
    let words = attest_words(k, entry, c, rel);
    let digest = attest_digest_at(k, entry.token.chain_id, bond_contract, &words);
    match rec.signer(&digest, &x.release_attest_sig) {
        Some(who) if who == c.payee && who != [0u8; 20] => AttestVerdict::Valid,
        _ => AttestVerdict::WrongSigner,
    }
}
