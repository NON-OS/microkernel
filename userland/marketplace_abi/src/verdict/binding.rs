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

//! Whether a listing's payee really bound its publisher key and bond. The
//! market reports this beside the commerce block; like every gate it is
//! advisory, and the chain is what finally holds the payee to it.

use nonos_nox_config::{address, BOND};

use super::binding_kind::BindingVerdict;
use super::digest::binding_digest_at;
use super::recover::Recover;
use crate::ids::{bond_id, listing_id, Keccak};
use crate::types::MarketplaceEntry;

/// Checked against the bond contract the config holds for the token's chain.
/// What can be judged without a domain is judged first, so a wrong bond is
/// reported as that even before any bond contract is deployed.
pub fn binding_verdict(
    k: &impl Keccak,
    rec: &impl Recover,
    entry: &MarketplaceEntry,
) -> BindingVerdict {
    let Some(c) = entry.commerce.as_ref() else { return BindingVerdict::NoCommerce };
    if c.bond_id != bond_id(k, &c.payee, &entry.publisher_pubkey) {
        return BindingVerdict::WrongBond;
    }
    match address(entry.token.chain_id, BOND.contract) {
        Ok(bond) => binding_verdict_at(k, rec, entry, bond),
        Err(_) => BindingVerdict::NoDomain,
    }
}

pub fn binding_verdict_at(
    k: &impl Keccak,
    rec: &impl Recover,
    entry: &MarketplaceEntry,
    bond_contract: [u8; 20],
) -> BindingVerdict {
    let Some(c) = entry.commerce.as_ref() else { return BindingVerdict::NoCommerce };
    let key = &entry.publisher_pubkey;
    if c.bond_id != bond_id(k, &c.payee, key) {
        return BindingVerdict::WrongBond;
    }
    let listing = listing_id(k, key, &entry.listing_id);
    let chain = entry.token.chain_id;
    let digest = binding_digest_at(k, chain, bond_contract, [key, &listing, &c.bond_id]);
    match rec.signer(&digest, &c.binding_sig) {
        Some(who) if who == c.payee && who != [0u8; 20] => BindingVerdict::Valid,
        _ => BindingVerdict::WrongSigner,
    }
}
