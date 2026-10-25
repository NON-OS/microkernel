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

//! What the operator gives for one machine: its attestation document, the
//! AK the registrar activated, its host keys, its name, the fleet, the
//! release loaders it may boot, how long the certificate lasts and which
//! authority will sign it.

use crate::keys::PUBLIC_FILE_LEN;

pub struct Request<'a> {
    /// `NONOSATT` version 5 from `MkAttestDoc`, as the machine gave it.
    pub doc: &'a [u8],
    /// The challenge the document was asked for (`challenge`), which names
    /// these host keys and a fresh nonce.
    pub challenge: [u8; 32],
    /// The AK whose residency the registrar proved through the EK, x then y.
    pub ak: [u8; 64],
    /// The machine's nsh host keys, Ed25519 then ML-DSA-65.
    pub host_keys: &'a [u8; PUBLIC_FILE_LEN],
    pub host: &'a [u8],
    pub fleet: [u8; 16],
    /// The Authenticode digests of the release loaders this machine may run.
    pub loaders: &'a [[u8; 32]],
    /// The boot applications the operator expects besides the loader.
    pub boot_apps: &'a [[u8; 32]],
    pub not_after_ms: u64,
    pub now_ms: u64,
    pub authority_id: [u8; 32],
}
