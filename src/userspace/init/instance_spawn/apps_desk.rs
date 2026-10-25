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

use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

#[cfg(feature = "nonos-capsule-clock")]
pub(super) fn spawn_clock() -> Result<u32, SpawnError> {
    crate::userspace::capsule_clock::spawn_clock_instance()
}

#[cfg(not(feature = "nonos-capsule-clock"))]
pub(super) fn spawn_clock() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}

#[cfg(feature = "nonos-capsule-about")]
pub(super) fn spawn_about() -> Result<u32, SpawnError> {
    crate::userspace::capsule_about::spawn_about_instance()
}

#[cfg(not(feature = "nonos-capsule-about"))]
pub(super) fn spawn_about() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}

#[cfg(feature = "nonos-capsule-snake")]
pub(super) fn spawn_snake() -> Result<u32, SpawnError> {
    crate::userspace::capsule_snake::spawn_snake_instance()
}

#[cfg(not(feature = "nonos-capsule-snake"))]
pub(super) fn spawn_snake() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}

#[cfg(feature = "nonos-capsule-wallet-nonos")]
pub(super) fn spawn_wallet_nonos() -> Result<u32, SpawnError> {
    crate::userspace::capsule_wallet_nonos::spawn_wallet_nonos_instance()
}

#[cfg(not(feature = "nonos-capsule-wallet-nonos"))]
pub(super) fn spawn_wallet_nonos() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}

#[cfg(feature = "nonos-capsule-file-manager")]
pub(super) fn spawn_file_manager() -> Result<u32, SpawnError> {
    crate::userspace::capsule_file_manager::spawn_file_manager_instance()
}

#[cfg(not(feature = "nonos-capsule-file-manager"))]
pub(super) fn spawn_file_manager() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}
