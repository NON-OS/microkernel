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

use super::hashers::Keccak;
use crate::codec::release_signing_bytes;
use crate::types::CapsuleRelease;

/// keccak256 of the release's v3 signing bytes: what NoxListingRegistry
/// records beside the release id. `None` for a schema 2 release, which was
/// never signed under v3 and cannot be published there.
pub fn release_hash(k: &impl Keccak, release: &CapsuleRelease) -> Option<[u8; 32]> {
    release.ext.as_ref()?;
    Some(k.keccak256(&release_signing_bytes(release)))
}
