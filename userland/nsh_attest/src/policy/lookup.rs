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

//! The lookups the checks make in a policy.

use super::{Authority, Policy, Release};

impl Policy {
    pub fn fleet_id(&self) -> &[u8; 16] {
        &self.fleet_id
    }

    /// The authority with this key id.
    pub fn authority(&self, id: &[u8; 32]) -> Option<&Authority> {
        self.authorities.iter().find(|a| a.id == *id)
    }

    /// The one release whose kernel this PCR 9 names. `new` refused two
    /// releases sharing one.
    pub fn release(&self, pcr9: &[u8; 32]) -> Option<&Release> {
        self.releases.iter().find(|r| r.pcr9 == *pcr9)
    }

    pub fn releases(&self) -> &[Release] {
        &self.releases
    }

    /// Whether an enrolment, by the SHA-512 of its certificate, is revoked.
    pub fn revoked(&self, cert_digest: &[u8; 64]) -> bool {
        self.revoked.iter().any(|r| r == cert_digest)
    }
}
