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

//! A parsed Ed25519 certificate, still unverified.

use super::super::constants::SIGNATURE_BYTES;

pub struct EdCert<'a> {
    pub cert_type: u8,
    /// Hours since the epoch, as the wire carries it.
    pub expiry_hours: u32,
    /// The key this certificate is about.
    pub certified_key: [u8; 32],
    /// The signing key an extension named, if one did.
    pub signed_with: Option<[u8; 32]>,
    /// Everything the signature covers: the certificate bar the signature.
    pub signed: &'a [u8],
    pub signature: [u8; SIGNATURE_BYTES],
}
