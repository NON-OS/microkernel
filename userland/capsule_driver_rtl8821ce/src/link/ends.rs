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

//! How an association ends, and what it leaves for the loop.

use nonos_wifi_core::dot11::auth::deauth_frame;
use nonos_wifi_core::station::Loss;

use super::buffers::RxBuffers;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    /// Tell the access point the station is leaving (reason 3, "deauthenticated
    /// because sending station is leaving"), so it does not hold the
    /// association, and with management frame protection does not have to
    /// probe for it before the station can join again. Protected under the
    /// pairwise key when protection is on. Best effort: a full ring drops it.
    pub fn send_deauth(&mut self) {
        if !self.station.is_associated() {
            return;
        }
        let frame = deauth_frame(self.station.mac(), self.station.bssid(), 0, 3);
        let frame = if self.pmf { self.station.protect_mgmt(&frame) } else { Some(frame) };
        if let Some(f) = frame {
            self.send_raw(&f);
        }
    }

    /// A group key the AP delivered, held until the CAM has it.
    pub fn group_key(&self) -> Option<(u8, &[u8; 16])> {
        self.new_group_key.as_ref().map(|(id, key)| (*id, &key.0))
    }

    /// The CAM has the group key: wiped where it lies.
    pub fn clear_group_key(&mut self) {
        self.new_group_key = None;
    }

    /// The reason code of a deauthentication the link took since the last call.
    pub fn take_left(&mut self) -> Option<u16> {
        self.left.take()
    }

    /// Whether the access point went unheard since the last call.
    pub fn take_silent(&mut self) -> bool {
        core::mem::take(&mut self.silent)
    }

    /// Note why the link ended, for the serving loop to close the session.
    pub(super) fn lost(&mut self, why: Loss) {
        self.history.lost();
        match why {
            Loss::Left(reason) | Loss::Unanswered(reason) => self.left = Some(reason),
            Loss::Silent => self.silent = true,
        }
    }
}
