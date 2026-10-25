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
//! The type string, the domain and the verifying contract are
//! nonos_nox_config's, so a redeploy changes that crate and not this one.

use nonos_nox_config::{Chain, Deployment, TypedData};

use super::field::Field;

#[derive(Debug)]
pub struct Entry {
    /// The number a request names it by.
    pub id: u16,
    /// The struct, its typehash and its domain, as the config pins them;
    /// wallet_proofs checks `fields` says what its type string says.
    pub typed: &'static TypedData,
    pub fields: &'static [Field],
    /// The service names whose owner may ask, looked up in the registry
    /// on every request.
    pub callers: &'static [&'static [u8]],
}

impl Entry {
    /// The contract this entry's domain verifies against on `chain`, as
    /// `deployment` places it. None is a contract not deployed there, and
    /// refuses.
    pub fn contract(
        &self,
        chain: Chain,
        deployment: fn(Chain) -> &'static Deployment,
    ) -> Option<[u8; 20]> {
        deployment(chain).get(self.typed.domain.contract)
    }
}
