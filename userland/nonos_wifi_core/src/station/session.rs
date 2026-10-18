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

//! What the association's handshake said of the network, and how a link
//! ends when the station notices it is gone.

use super::{LinkStation, Loss};

impl LinkStation {
    /// Message 3's Transition Disable bitmap for this association, 0 when it
    /// sent none (bit 0: the network is WPA3-Personal only from now on).
    pub fn transition_disable(&self) -> u8 {
        self.supplicant.as_ref().and_then(|s| s.transition_disable()).unwrap_or(0)
    }

    /// End the link for `why`.
    pub(super) fn lose(&mut self, why: Loss) -> Loss {
        self.deassociate();
        why
    }
}
