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

//! What the Wi-Fi driver and net_core last said: kept apart from the panel's
//! own state so the reasons a link is down can be told on the page.

use crate::wifi::{DataPath, DriverStage, NetStatus, ScanStats};

pub struct WifiDiag {
    /// The driver's radio bring-up stage from the last query, if it answered.
    pub stage: Option<DriverStage>,
    /// The driver's receive-pipeline counters from the last scan, so the panel can
    /// show where reception stops when no networks come back.
    pub stats: ScanStats,
    /// The driver's data-path frame counts from the last refresh, so a stuck DHCP
    /// is legible: no TX means net_core handed nothing down, RX without parsed
    /// means the frames never decrypt.
    pub datapath: Option<DataPath>,
    /// What net_core reports: down, bound-but-no-address, or a bound lease. Splits
    /// "the stack never started" from "started but got no address".
    pub net: NetStatus,
    /// When `net` was last asked of the DHCP client, on the uptime clock
    /// (wifi/net_poll.rs).
    pub net_polled_ms: Option<i64>,
    /// The wired card's station source (wifi/station.rs), once answered.
    pub station: Option<u32>,
}

impl WifiDiag {
    pub fn new() -> Self {
        WifiDiag {
            stage: None,
            stats: ScanStats::default(),
            datapath: None,
            net: NetStatus::NoService,
            net_polled_ms: None,
            station: None,
        }
    }
}
