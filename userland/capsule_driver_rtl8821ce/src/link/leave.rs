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

//! Which deauthentication the link takes.

use nonos_wifi_core::dot11::auth::parse_leave;

use super::buffers::RxBuffers;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    // A deauthentication or disassociation from the AP takes the link down,
    // unless management frame protection is on: then an unprotected one may
    // be forged, and the station does not process protected management frames,
    // so it relies on the AP's own teardown instead.
    pub(super) fn note_leave(&mut self, frame: &[u8]) -> bool {
        if self.pmf {
            return false;
        }
        let Some(reason) = parse_leave(frame, &self.station.mac(), &self.station.bssid()) else {
            return false;
        };
        self.station.deassociate();
        self.left = Some(reason);
        true
    }
}
