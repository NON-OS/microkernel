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

//! Where an install stands, as `mk_app_install_status` reports it. Not the
//! market's wire, but every client of the market shows it beside the
//! market's answers, so it is read in one place.

/// The kernel's answer, decoded.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage {
    /// Nothing asked since the system started.
    Idle,
    Queued,
    Installing,
    Installed,
    /// Init would not start the installer.
    Refused,
    /// The personality is taking what the install put down away again.
    Removing,
    /// Taken away: as good as never installed.
    Removed,
    /// The installer stopped, with its reason code (`install::Why` in the
    /// Linux personality).
    Failed(u8),
}

impl Stage {
    /// 0 idle, 1 queued, 2 installing, 3 installed, 4 refused, 5 removing,
    /// 6 removed, and 16 plus the installer's exit code for a failure, an
    /// install's or an uninstall's. A negative answer is an
    /// errno for the question, not a stage, and reads as idle.
    pub fn of(code: i64) -> Stage {
        match code {
            1 => Stage::Queued,
            2 => Stage::Installing,
            3 => Stage::Installed,
            4 => Stage::Refused,
            5 => Stage::Removing,
            6 => Stage::Removed,
            c if c >= 16 => Stage::Failed((c - 16).clamp(0, 255) as u8),
            _ => Stage::Idle,
        }
    }

    /// Still moving, so worth asking again.
    pub fn pending(self) -> bool {
        matches!(self, Stage::Queued | Stage::Installing | Stage::Removing)
    }
}
