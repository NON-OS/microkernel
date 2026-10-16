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

//! What asking again can do for a listing at each stage it stands at.

use nonos_market_proto::reason;
use nonos_market_proto::reason::WHY_NOT_INSTALLED;

use super::{Progress, NO_VOLUME};

impl Progress {
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
}
