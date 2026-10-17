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
//! The open link: its station, what it holds for net_core, and its counts.

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use nonos_wifi_core::key::HeldKey;
use nonos_wifi_core::station::LinkStation;

/// Ethernet frames held for net_core between its polls.
pub const RX_READY: usize = 32;
/// `WLAN_REASON_DEAUTH_LEAVING`.
pub(super) const REASON_LEAVING: u16 = 3;
pub(super) const MIC_LEN: usize = 8;
/// The link's counts, for the link and status replies.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct LinkStats {
    pub tx_ok: u32,
    pub tx_drop: u32,
    pub rx_frames: u32,
    pub rx_eth: u32,
    /// Data frames the station's checks refused.
    pub rx_refused: u32,
    /// Ethernet frames dropped with net_core not polling.
    pub rx_full: u32,
    pub rekeys: u32,
}

pub struct Link {
    pub station: LinkStation,
    pub ready: VecDeque<Vec<u8>>,
    pub pmf: bool,
    /// The reason code of the access point's deauthentication, taken as it
    /// came or after an unanswered SA Query.
    pub left: Option<u16>,
    /// The access point went unheard and answered no probe.
    pub silent: bool,
    /// A group key the access point delivered, held until the firmware has
    /// it; wiped where it lies when replaced or cleared.
    pub new_group_key: Option<(u8, HeldKey)>,
    /// A rekeyed pairwise key, likewise held until the firmware has it.
    pub new_pairwise_key: Option<HeldKey>,
    pub stats: LinkStats,
    /// The rate words for data frames and for handshake and management ones.
    pub data_rate: u32,
    pub mgmt_rate: u32,
}
