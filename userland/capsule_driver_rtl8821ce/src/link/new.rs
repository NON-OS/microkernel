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

//! A link built over mapped DMA, and its counts.

use nonos_wifi_core::link_history::LinkHistory;
use nonos_wifi_core::station::LinkStation;

use super::buffers::RxBuffers;
use super::counts::LinkStats;
use super::rings::program_rings;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::rx::ring::{RxState, RX_DESC_COUNT};
use crate::tx::ring::{TxState, TX_DESC_COUNT};
use crate::tx::stall::TxWatch;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    /// Build the link over already-mapped DMA and program both rings into the
    /// card. The station starts unassociated, so the link stays down until
    /// `associate` records the AP the supplicant authenticated against.
    pub fn new(
        mmio: M,
        tx_ring: D,
        tx_buffers: D,
        rx_ring: D,
        rx_buffers: RB,
        our_mac: [u8; 6],
    ) -> Self {
        // Bring-up pointed the card at these rings before its engines ran
        // (`engines_on`); program them again and reset the DMA interface, so
        // the card and `rx_state` both start from slot zero. Without the reset
        // the card never advances its receive write index.
        crate::mac::set_dma_burst(&mmio);
        program_rings(&mmio, &tx_ring, &rx_ring, rx_buffers.device_addr());
        Self {
            mmio,
            tx_ring,
            tx_buffers,
            tx_state: TxState::new(TX_DESC_COUNT),
            tx_watch: TxWatch::default(),
            rx_ring,
            rx_buffers,
            rx_state: RxState::new(RX_DESC_COUNT),
            station: LinkStation::new(our_mac),
            stats: LinkStats::default(),
            pmf: false,
            new_group_key: None,
            new_pairwise_key: None,
            left: None,
            silent: false,
            history: LinkHistory::NEW,
            address: None,
            ps: crate::ps::Ps::default(),
            beacon: None,
            rfe: 0,
        }
    }
}
