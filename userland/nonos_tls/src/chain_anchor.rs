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

//! Where a served chain ends: at a pinned key, or at a root that signed it.

// The top served certificate is trusted if it is itself a pinned anchor (the
// server sent the root, or its SPKI is directly pinned), otherwise if a trusted
// root issued it and the signature verifies against that root's key.
pub(crate) fn anchor(top: &[u8]) -> super::crypto_port::Checked {
    if let Some(top_spki) = super::cert_spki::cert_spki(top) {
        if let Some(h) = super::hash_sha256::hash_sha256(top_spki) {
            if super::roots::is_trusted_spki_hash(&h) {
                return Ok(true);
            }
        }
    }
    let Some(issuer) = super::cert_issuer::cert_issuer(top) else {
        return Ok(false);
    };
    let Some(root_spki) = super::roots::find_spki_by_subject(issuer) else {
        return Ok(false);
    };
    super::verify_link::verify_link(top, root_spki)
}
