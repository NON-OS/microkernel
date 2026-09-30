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

//! Keying the data volume with a passphrase, so it opens on any machine
//! and without a TPM. The passphrase is stretched with Argon2id under a
//! random salt; the result seals a random volume key into the key header.
//! A passphrase that does not open the sealed key is refused before any
//! volume sector is read, and nothing is formatted.

use super::error::VolumeError;
use super::passphrase_create::create_keyed;
use super::passphrase_open::open_keyed;
use super::plan_read::read_plan;
use super::state::VOLUME;

/// Create a passphrase-keyed volume over a blank header ring (`create`),
/// or open the one the key header names. The caller wipes `passphrase`.
pub fn passphrase_volume(create: bool, passphrase: &[u8]) -> Result<(), VolumeError> {
    if VOLUME.read().is_some() {
        return Err(VolumeError::AlreadyOpen);
    }
    let plan = read_plan()?;
    crate::fs::cryptoblock::set_window(plan.volume_base, plan.volume_sectors)
        .map_err(VolumeError::Window)?;
    if create {
        create_keyed(passphrase, &plan)
    } else {
        open_keyed(passphrase, &plan)
    }
}
