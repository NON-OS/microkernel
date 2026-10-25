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

use nonos_market_proto::reason::WHY_NO_VOLUME;
use nonos_market_proto::Stage;

#[path = "progress_ask.rs"]
mod progress_ask;
#[path = "progress_button.rs"]
mod progress_button;

/// The installer's reason code for a machine with no data volume.
pub const NO_VOLUME: u8 = WHY_NO_VOLUME;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Progress {
    /// Nothing asked since the system started.
    Idle,
    Queued,
    Installing,
    Installed,
    /// The system would not start the installer.
    Refused,
    /// The installer stopped, with its reason code: an install's, or an
    /// uninstall's (`Listing::removing` says which was asked last).
    Failed(u8),
    /// What the install put down is being taken away.
    Removing,
    /// Taken away: as good as never installed, so it offers Install again.
    Removed,
}

impl Progress {
    /// From `mk_app_install_status`, decoded where the Terminal's `market`
    /// decodes it too.
    pub fn of(code: i64) -> Progress {
        match Stage::of(code) {
            Stage::Idle => Progress::Idle,
            Stage::Queued => Progress::Queued,
            Stage::Installing => Progress::Installing,
            Stage::Installed => Progress::Installed,
            Stage::Refused => Progress::Refused,
            Stage::Failed(why) => Progress::Failed(why),
            Stage::Removing => Progress::Removing,
            Stage::Removed => Progress::Removed,
        }
    }
}
