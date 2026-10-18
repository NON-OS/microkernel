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

//! One kind of message the keyring will sign, everything about it fixed.

use super::chains::CHAINS;
use super::domain::Domain;
use super::field::Field;

#[derive(Debug)]
pub struct Entry {
    /// The number a request names it by.
    pub id: u16,
    /// Hashed on every signature, so the hash cannot drift from the text a
    /// reviewer reads; wallet_proofs pins the result.
    pub type_string: &'static str,
    pub domain: &'static Domain,
    pub fields: &'static [Field],
    /// The service names whose owner may ask, looked up in the registry
    /// on every request.
    pub callers: &'static [&'static [u8]],
    /// The verifying contract on each of CHAINS, in that order. None is a
    /// contract not deployed there, and refuses.
    pub contracts: [Option<[u8; 20]>; CHAINS.len()],
}

impl Entry {
    /// The contract this entry verifies against on `chain`, if the chain is
    /// one of ours and the contract is deployed on it.
    pub fn contract(&self, chain: u64) -> Option<[u8; 20]> {
        let at = CHAINS.iter().position(|&c| c == chain)?;
        self.contracts[at]
    }
}
