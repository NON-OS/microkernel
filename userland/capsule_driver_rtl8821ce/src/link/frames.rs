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

//! Raw frames off the ring and onto it.

use super::buffers::RxBuffers;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::rx::desc::RxInfo;
use crate::rx::fcs::without_fcs;
use crate::rx::poll_one_info;
use crate::rx::signal::signal_dbm;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    /// Pull the next received 802.11 frame off the ring unparsed, for the scan to
    /// read beacons the data path would otherwise drop. Returns its length
    /// without the FCS, or `None` when nothing is queued.
    pub fn poll_raw(&mut self, out: &mut [u8]) -> Option<usize> {
        self.poll_frame(out).map(|(n, _)| n)
    }

    /// As `poll_raw`, with the frame's signal in dBm where the card measured
    /// one, for the scan and the hunt to rank networks by.
    pub fn poll_heard(&mut self, out: &mut [u8]) -> Option<(usize, Option<i8>)> {
        let (n, info) = self.poll_info(out)?;
        Some((n, info.phy.and_then(|p| signal_dbm(p, self.rfe))))
    }

    /// The board's RF front-end option, from the efuse.
    pub fn set_rfe(&mut self, rfe: u8) {
        self.rfe = rfe;
    }

    // The next received frame without its FCS, and whether the chip decrypted it.
    pub(super) fn poll_frame(&mut self, out: &mut [u8]) -> Option<(usize, bool)> {
        self.poll_info(out).map(|(n, info)| (n, info.decrypted()))
    }

    fn poll_info(&mut self, out: &mut [u8]) -> Option<(usize, RxInfo)> {
        let (n, info) = poll_one_info(
            &self.mmio,
            &self.rx_ring,
            &mut self.rx_buffers,
            &mut self.rx_state,
            out,
        )?;
        Some((without_fcs(n)?, info))
    }
}
