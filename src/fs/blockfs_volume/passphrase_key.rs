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

//! Stretching a passphrase into the key that seals the volume key.

use super::error::VolumeError;
use super::key_header::Sealed;
use crate::crypto::util::argon2::argon2id;

/// Argon2id of `passphrase` under the header's salt and parameters. The
/// derivation runs for as long as its parameters make it; TLB shootdowns
/// are answered between its segments so other cores are not held.
pub(super) fn stretch(passphrase: &[u8], sealed: &Sealed) -> Result<[u8; 32], VolumeError> {
    let mut kek = [0u8; 32];
    let mut between = || crate::smp::serve_shootdowns();
    argon2id(passphrase, &sealed.salt, sealed.params, &mut kek, &mut between).map_err(|e| {
        crate::log::warn!("[DATA] Argon2id refused: {:?}", e);
        VolumeError::Stretch(e)
    })?;
    Ok(kek)
}
