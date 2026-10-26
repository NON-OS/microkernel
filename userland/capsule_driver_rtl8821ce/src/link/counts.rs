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

//! The link's counts.

use super::buffers::RxBuffers;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;

/// Data-path frame counts, read over the control protocol so a laptop with no
/// serial port can see where the DHCP path stops: TX frames net_core handed us
/// versus RX frames the ring returned and how many parsed to Ethernet.
#[derive(Clone, Copy, Default)]
pub struct LinkStats {
    pub tx_ok: u32,
    pub tx_drop: u32,
    pub rx_ring: u32,
    pub rx_eth: u32,
    /// net_core link-protocol requests answered. Nonzero proves net_core is
    /// alive and probing this interface; zero means the stack never reached it.
    pub netif_reqs: u32,
    /// Frames dropped for a CRC or ICV error; a climbing count while a lease is
    /// awaited points at a reply that arrives but does not decrypt.
    pub rx_err: u32,
    /// Data frames the station's checks refused (another BSS, unprotected,
    /// a replay, undecryptable).
    pub rx_refused: u32,
    /// Group key handshakes answered on the link.
    pub rekeys: u32,
    /// Times the rings were reset because the card stopped reading the
    /// transmit ring.
    pub tx_resets: u32,
    /// Probes and SA Queries the station's watch sent: the AP went quiet.
    pub watch_sends: u32,
}

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    /// The data-path frame counts for the control-protocol status reply. The
    /// receive-error count lives with the ring state, so fold it in here.
    pub fn stats(&self) -> LinkStats {
        LinkStats { rx_err: self.rx_state.err_drops, ..self.stats }
    }

    /// Count one answered net_core link-protocol request.
    pub fn note_netif_req(&mut self) {
        self.stats.netif_reqs = self.stats.netif_reqs.wrapping_add(1);
    }
}
