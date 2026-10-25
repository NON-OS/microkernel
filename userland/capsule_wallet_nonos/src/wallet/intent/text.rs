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
 * Text for an intent's review, every value whole: contract names, an id
 * as all its hex (hex_id), an address in its EIP-55 spelling.
 */

use alloc::format;
use alloc::string::String;

use nonos_marketplace_abi::{address_text, Keccak};
use nonos_nox_config::Contract;

pub use super::super::hex_id::hex;

/// `a` in full, 0x and its EIP-55 spelling (marketplace_abi's).
pub fn addr_text(k: &impl Keccak, a: &[u8; 20]) -> String {
    address_text(k, a).iter().map(|c| char::from(*c)).collect()
}

/// A contract by name and its address in full, or why there is none.
pub fn placed(k: &impl Keccak, c: Contract, a: Option<[u8; 20]>) -> String {
    match a {
        Some(a) => format!("{} at {}", contract_name(c), addr_text(k, &a)),
        None => format!("{}: that contract is not on this network yet", contract_name(c)),
    }
}

pub fn contract_name(c: Contract) -> &'static str {
    match c {
        Contract::NoxToken => "the NOX token",
        Contract::Settlement => "NONOS Settlement",
        Contract::Escrow => "NONOS Escrow",
        Contract::PublisherBond => "NONOS Bond",
        Contract::ListingRegistry => "NONOS Listing registry",
        Contract::ProofAnchor => "NONOS Proof anchor",
        Contract::ShieldPool => "the Shield pool",
        Contract::AmountPolicy => "the Shield pool's amount policy",
    }
}
