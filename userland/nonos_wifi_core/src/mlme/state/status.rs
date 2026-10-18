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

//! Where the join stands, why it failed, and what it chose.

use super::*;

impl Mlme {
    pub fn state(&self) -> MlmeState {
        self.state
    }

    /// Why the join failed, once `Failed`; while an SAE exchange runs, why
    /// its last frame was discarded, for a driver whose deadline ends it.
    pub fn failure(&self) -> Option<MlmeFailure> {
        let sae = || self.sae.as_ref()?.failure().map(MlmeFailure::Sae);
        self.failure.or_else(sae)
    }

    /// Message 3's Transition Disable bits, once the handshake took them.
    pub fn transition_disable(&self) -> Option<u8> {
        self.supplicant.as_ref().and_then(Supplicant::transition_disable)
    }

    /// The security chosen from the beacon, once a BSS was selected.
    pub fn selection(&self) -> Option<Selection> {
        self.selection
    }
}
