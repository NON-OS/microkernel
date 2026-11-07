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

//! The user-files broker (capsule_user_files) comes up with the desktop's
//! services: it keeps the person's Documents and Downloads on the data
//! volume, and every app that opens one asks it.

#[cfg(feature = "nonos-capsule-user-files")]
pub(super) fn spawn() {
    use crate::userspace::capsule_user_files as c;
    super::boot::capsule("USER-FILES", "user_files", c::spawn_user_files_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-user-files"))]
pub(super) fn spawn() {}
