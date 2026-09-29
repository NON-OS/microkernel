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

//! Where an install this window asked for stands, as the system reports it,
//! and how that reads to a person.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Progress {
    /// Nothing asked since the system started.
    Idle,
    Queued,
    Installing,
    Installed,
    /// The system would not start the installer.
    Refused,
    /// The installer stopped, with its reason code.
    Failed(u8),
}

impl Progress {
    /// From `mk_app_install_status`.
    pub fn of(code: i64) -> Progress {
        match code {
            1 => Progress::Queued,
            2 => Progress::Installing,
            3 => Progress::Installed,
            4 => Progress::Refused,
            c if c >= 16 => Progress::Failed((c - 16).clamp(0, 255) as u8),
            _ => Progress::Idle,
        }
    }

    /// Still moving, so worth asking again.
    pub fn pending(self) -> bool {
        matches!(self, Progress::Queued | Progress::Installing)
    }

    pub fn button(self, ready: bool) -> &'static [u8] {
        match self {
            Progress::Idle if ready => b"Install",
            Progress::Idle => b"Details",
            Progress::Queued => b"Queued",
            Progress::Installing => b"Installing",
            Progress::Installed => b"Open",
            Progress::Refused | Progress::Failed(_) => b"Retry",
        }
    }
}
