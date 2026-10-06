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

//! Certificates from the field and made for the proofs.

/// The chain a re-signing gateway served for nonos.software: leaf, issuer and
/// the gateway's own root, each RSA-2048. Its root is in no trust store, so
/// the chain has to be refused.
pub const GATEWAY: [&[u8]; 3] =
    [include_bytes!("gw0.der"), include_bytes!("gw1.der"), include_bytes!("gw2.der")];

/// A P-384 authority, and a P-256 certificate it signed whose many names make
/// its to-be-signed part longer than 1,516 bytes.
pub const P384_AUTHORITY: &[u8] = include_bytes!("p384_authority.der");
pub const P384_MANY_NAMES: &[u8] = include_bytes!("p384_many_names.der");
