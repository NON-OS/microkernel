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

//! One authority signature line, as the footer carries it.

extern crate alloc;

use alloc::vec::Vec;

/// One authority's signature over the consensus.
pub struct Signature {
    /// Which digest the signature is over. A flavoured consensus uses `sha256`;
    /// a line naming no algorithm means sha1, which this capsule will not accept
    pub sha256: bool,
    /// SHA-1 of the signing authority's identity key, which has to match one of
    /// the hardcoded v3 identities.
    pub identity: [u8; 20],
    /// SHA-1 of the signing key that produced it, which has to match the key in
    /// that authority's certificate.
    pub signing_key: [u8; 20],
    pub bytes: Vec<u8>,
}
