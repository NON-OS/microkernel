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

//! An association on the link: its keys, its end, and what it leaves.

use nonos_wifi_core::station::Ccmp;
use nonos_wifi_core::wpa::supplicant::Supplicant;

use super::buffers::RxBuffers;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    /// Record association to `bssid`, encrypting transmit frames in software with
    /// `ptk` while the radio decrypts receive frames from its key cache. This chip
    /// does not encrypt transmit frames in hardware but does decrypt receive
    /// frames, so the transmit path mirrors the plaintext handshake path and the
    /// receive path leaves the sec engine on.
    pub fn associate(&mut self, bssid: [u8; 6], ptk: [u8; 16]) {
        self.station.associate(bssid, Ccmp::SoftwareTxHwRx { tk: ptk });
        self.pmf = false;
        self.new_group_key = None;
        self.left = None;
    }

    /// Hand the link the supplicant the join finished with, so it answers the
    /// AP's group key handshakes and repeated message 3s, and decrypts group
    /// frames the chip does not.
    pub fn set_supplicant(&mut self, sup: Supplicant) {
        self.pmf = sup.pmf();
        self.station.set_supplicant(sup);
    }

    /// Drop the association; the link reports down and refuses to transmit until
    /// it associates again.
    pub fn deassociate(&mut self) {
        self.station.deassociate();
    }
}
