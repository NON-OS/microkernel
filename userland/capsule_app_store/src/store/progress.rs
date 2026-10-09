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

use nonos_market_proto::reason::{WHY_NOT_INSTALLED, WHY_NO_VOLUME};
use nonos_market_proto::{reason, Stage};

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

    /// The installer stopped because no disk here carries NONOS, so there is
    /// no data volume at all (ENODEV). A live stick's volume is in memory and
    /// is not this. Asking again cannot change it, so Enter does nothing and
    /// the button says why instead of offering a retry.
    pub fn needs_installed_system(self) -> bool {
        self == Progress::Failed(NO_VOLUME)
    }

    /// A stopped install that asking again could finish. No NONOS disk, or a
    /// system built without what the install needs, is not one: no Retry
    /// is offered for what can only fail the same way again.
    pub fn retryable(self) -> bool {
        match self {
            Progress::Refused => true,
            Progress::Failed(why) => reason(why).retry,
            _ => false,
        }
    }

    /// Still moving, so worth asking again.
    pub fn pending(self) -> bool {
        matches!(self, Progress::Queued | Progress::Installing | Progress::Removing)
    }

    /// As good as never installed: nothing asked, taken away, or an
    /// uninstall that found nothing installed. Install is offered again.
    pub fn uninstalled(self) -> bool {
        matches!(self, Progress::Idle | Progress::Removed | Progress::Failed(WHY_NOT_INSTALLED))
    }

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
