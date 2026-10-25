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

//! The local test chain's throwaway root (P5), for the TLS proxy a developer
//! runs in front of a local node. Its private key is public in
//! tools/devnet/tls, so only a `devnet` build, which only development images
//! take, may compile this in; tools/nix/release_names.nix fails a release
//! capsule that holds its name or its key's hash.

/// The root's SubjectDN, CN NONOS-devnet-throwaway-root: what the served
/// leaf names as its issuer.
pub static DEV_ROOT_SUBJECT: &[u8] =
    include_bytes!("../../../../tools/devnet/tls/root_subject.der");
/// The root's SubjectPublicKeyInfo, which the leaf's signature is checked by.
pub static DEV_ROOT_SPKI: &[u8] = include_bytes!("../../../../tools/devnet/tls/root_spki.der");

/// SHA-256 of DEV_ROOT_SPKI, the form every other root is pinned in. A static,
/// so the bytes sit whole in the binary where the release check looks.
pub static DEV_ROOT_SPKI_SHA256: [u8; 32] = [
    252, 167, 24, 79, 224, 2, 228, 217, 89, 134, 108, 251, 184, 226, 115, 243, 150, 0, 49, 146, 68,
    87, 252, 253, 177, 5, 68, 183, 247, 164, 161, 45,
];

/// Whether `h` is the throwaway root's SPKI hash.
pub fn is_dev_root(h: &[u8; 32]) -> bool {
    h == &DEV_ROOT_SPKI_SHA256
}

/// The throwaway root's key, when `subject` is its name.
pub fn dev_spki_by_subject(subject: &[u8]) -> Option<&'static [u8]> {
    (subject == DEV_ROOT_SUBJECT).then_some(DEV_ROOT_SPKI)
}
