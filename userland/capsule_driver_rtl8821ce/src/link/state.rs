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

//! The link's rings, station and counts.

use nonos_wifi_core::address::AddressKey;
use nonos_wifi_core::link_history::LinkHistory;
use nonos_wifi_core::station::LinkStation;

use super::counts::LinkStats;
use super::RxBuffers;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::rx::ring::{RxState, RX_DESC_COUNT};
use crate::tx::ring::TxState;

/// Most frames one receive poll drains looking for a data frame. Twice the ring
/// so a full backlog of beacons is cleared in a call, but bounded so a continuous
/// stream cannot spin it.
pub(super) const RX_DRAIN_MAX: usize = RX_DESC_COUNT as usize * 2;

/// The Ethernet data path net_core drives for this radio. It owns the transmit
/// and receive rings and one `LinkStation`; every WiFi specific (framing, CCMP,
/// the sequence number) lives in the station, so this is the thin ring shim the
/// shared `serve` loop expects. `D` is the descriptor/buffer DMA; `RB` the
/// readable receive buffers.
pub struct RtlLink<M: Mmio, D: DmaMem, RB: RxBuffers> {
    pub(super) mmio: M,
    pub(super) tx_ring: D,
    pub(super) tx_buffers: D,
    pub(super) tx_state: TxState,
    pub(super) rx_ring: D,
    pub(super) rx_buffers: RB,
    pub(super) rx_state: RxState,
    pub(super) station: LinkStation,
    pub(super) stats: LinkStats,
    /// Management frame protection is on for this association, so the
    /// station's own deauthentication goes protected.
    pub(super) pmf: bool,
    /// A group key the AP delivered on the link, for the serve loop to install.
    pub(super) new_group_key: Option<(u8, [u8; 16])>,
    /// The reason code of the access point's deauthentication, taken as it
    /// came or after an unanswered SA Query; the link is down.
    pub(super) left: Option<u16>,
    /// The access point went unheard and answered no probe; the link is down.
    pub(super) silent: bool,
    /// How the link last ended and the last join came out, for the link op.
    pub(super) history: LinkHistory,
    /// This boot's address key, drawn at the first join.
    pub(super) address: Option<AddressKey>,
}
