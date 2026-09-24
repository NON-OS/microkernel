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

//! One relay as the consensus describes it, before its microdescriptor arrives.

use crate::path::Flags;

/// Half a usable relay. The ntor key a handshake needs is only in the
/// microdescriptor `microdesc_digest` names, so an entry alone cannot be built
#[derive(Clone, Default)]
pub struct Entry {
    /// SHA-1 of the RSA identity key.
    pub rsa_identity: [u8; 20],
    pub address: [u8; 4],
    pub or_port: u16,
    /// SHA-256 of the microdescriptor, from the `m` line.
    pub microdesc_digest: [u8; 32],
    pub flags: Flags,
    pub weight: u32,
}

impl Entry {
    /// An all zero digest means no `m` line was seen and nothing can complete it.
    pub fn complete(&self) -> bool {
        self.or_port != 0
            && self.rsa_identity.iter().any(|b| *b != 0)
            && self.microdesc_digest.iter().any(|b| *b != 0)
    }
}
