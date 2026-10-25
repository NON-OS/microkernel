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

//! How the link last ended and how the last join came out, for the link op:
//! a loss is noted where the link ends (`ends`), a join and a request to
//! leave where the control family answers them.

use nonos_wifi_core::link_history::LinkHistory;

use super::buffers::RxBuffers;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    pub fn history(&self) -> LinkHistory {
        self.history
    }

    /// Message 3's Transition Disable bitmap for this association.
    pub fn transition_disable(&self) -> u8 {
        self.station.transition_disable()
    }

    /// A join finished with `code`, 0 for joined.
    pub fn note_join(&mut self, code: i32) {
        self.history.joined(code);
    }

    /// The person asked to leave the network.
    pub fn note_left(&mut self) {
        self.history.left();
    }
}
