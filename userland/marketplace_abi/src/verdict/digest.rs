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

//! The EIP-712 digest of a PublisherBinding under the bond contract's domain
//! on one chain, with every constant from `nonos_nox_config`.

use nonos_nox_config::{address, domain_words_at, ConfigError, BOND, PUBLISHER_BINDING};

use crate::ids::Keccak;

/// The binding the payee signs, for the bond contract at `bond_contract`.
pub fn binding_digest_at(
    k: &impl Keccak,
    chain_id: u64,
    bond_contract: [u8; 20],
    fields: [&[u8; 32]; 3],
) -> [u8; 32] {
    let separator = k.keccak256(&domain_words_at(&BOND, chain_id, bond_contract));
    let mut pre = [0u8; 128];
    pre[..32].copy_from_slice(&PUBLISHER_BINDING.typehash);
    for (i, f) in fields.iter().enumerate() {
        pre[32 * (i + 1)..32 * (i + 2)].copy_from_slice(*f);
    }
    let struct_hash = k.keccak256(&pre);
    let mut msg = [0u8; 66];
    msg[..2].copy_from_slice(&[0x19, 0x01]);
    msg[2..34].copy_from_slice(&separator);
    msg[34..].copy_from_slice(&struct_hash);
    k.keccak256(&msg)
}

/// The same, for the bond contract the config holds on this chain. Fields
/// are publisherKey, listingId and bondId, in the type string's order.
pub fn binding_digest(
    k: &impl Keccak,
    chain_id: u64,
    fields: [&[u8; 32]; 3],
) -> Result<[u8; 32], ConfigError> {
    let contract = address(chain_id, BOND.contract)?;
    Ok(binding_digest_at(k, chain_id, contract, fields))
}
