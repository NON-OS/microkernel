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
 * Text for an intent's review: contract names, and hex as a person reads
 * it (hex_id).
 */

use nonos_nox_config::Contract;

pub use super::super::hex_id::{hex, id};

pub fn contract_name(c: Contract) -> &'static str {
    match c {
        Contract::NoxToken => "the NOX token",
        Contract::Settlement => "NONOS Settlement",
        Contract::Escrow => "NONOS Escrow",
        Contract::PublisherBond => "NONOS Bond",
        Contract::ListingRegistry => "NONOS Listing registry",
        Contract::ProofAnchor => "NONOS Proof anchor",
        Contract::ShieldPool => "the Shield pool",
    }
}
