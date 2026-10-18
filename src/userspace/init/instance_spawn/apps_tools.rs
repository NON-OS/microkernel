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

#[cfg(feature = "nonos-capsule-terminal")]
pub(super) fn spawn_terminal() -> Result<u32, SpawnError> {
    crate::userspace::capsule_terminal::spawn_terminal_instance()
}

#[cfg(not(feature = "nonos-capsule-terminal"))]
pub(super) fn spawn_terminal() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}

#[cfg(feature = "nonos-capsule-browser")]
pub(super) fn spawn_browser() -> Result<u32, SpawnError> {
    crate::userspace::capsule_browser::spawn_browser_instance()
}

#[cfg(not(feature = "nonos-capsule-browser"))]
pub(super) fn spawn_browser() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}

#[cfg(feature = "nonos-capsule-text-editor")]
pub(super) fn spawn_text_editor() -> Result<u32, SpawnError> {
    crate::userspace::capsule_text_editor::spawn_text_editor_instance()
}

#[cfg(not(feature = "nonos-capsule-text-editor"))]
pub(super) fn spawn_text_editor() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}

#[cfg(feature = "nonos-capsule-settings")]
pub(super) fn spawn_settings() -> Result<u32, SpawnError> {
    crate::userspace::capsule_settings::spawn_settings_instance()
}

#[cfg(not(feature = "nonos-capsule-settings"))]
pub(super) fn spawn_settings() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}

#[cfg(feature = "nonos-capsule-calculator")]
pub(super) fn spawn_calculator() -> Result<u32, SpawnError> {
    crate::userspace::capsule_calculator::spawn_calculator_instance()
}

#[cfg(not(feature = "nonos-capsule-calculator"))]
pub(super) fn spawn_calculator() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}
