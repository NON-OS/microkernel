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

//! What a listing's button says at each stage it stands at.

use super::Progress;

impl Progress {
    pub fn button(self, ready: bool) -> &'static [u8] {
        match self {
            p if p.uninstalled() && ready => b"Install",
            Progress::Queued => b"Queued",
            Progress::Installing => b"Installing",
            Progress::Installed => b"Open",
            Progress::Removing => b"Removing",
            p if p.needs_installed_system() => b"Live boot",
            p if p.retryable() => b"Retry",
            Progress::Idle | Progress::Removed | Progress::Refused | Progress::Failed(_) => {
                b"Details"
            }
        }
    }
}
