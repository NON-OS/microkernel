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

#[cfg(feature = "nonos-capsule-process-manager")]
pub(super) fn spawn_process_manager() -> Result<u32, SpawnError> {
    crate::userspace::capsule_process_manager::spawn_process_manager_instance()
}

#[cfg(not(feature = "nonos-capsule-process-manager"))]
pub(super) fn spawn_process_manager() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}

#[cfg(feature = "nonos-capsule-audio-player")]
pub(super) fn spawn_audio_player() -> Result<u32, SpawnError> {
    crate::userspace::capsule_audio_player::spawn_audio_player_instance()
}

#[cfg(not(feature = "nonos-capsule-audio-player"))]
pub(super) fn spawn_audio_player() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}

#[cfg(feature = "nonos-capsule-video-player")]
pub(super) fn spawn_video_player() -> Result<u32, SpawnError> {
    crate::userspace::capsule_video_player::spawn_video_player_instance()
}

#[cfg(not(feature = "nonos-capsule-video-player"))]
pub(super) fn spawn_video_player() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}

#[cfg(feature = "nonos-capsule-install")]
pub(super) fn spawn_install() -> Result<u32, SpawnError> {
    crate::userspace::capsule_install::spawn_install_instance()
}
#[cfg(not(feature = "nonos-capsule-install"))]
pub(super) fn spawn_install() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}
