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

//! The WiFi settings panel's own pieces. The driver client, the scan result
//! and the saved networks live in `nonos_wifi_client`, shared with first-boot
//! setup and net_core. Here: the wireless adapters the broker discovered
//! (`adapters`, `interface`), the RTL8821CE's data-path counters (`datapath`),
//! and the address net_core bound (`lease`, `net_status`).

mod adapters;
mod datapath;
mod interface;
mod lease;
mod net_poll;
mod net_status;

pub use adapters::scan_adapters;
pub use datapath::{driver_datapath, DataPath};
pub use interface::{has_driver, WifiInterface};
pub use lease::{Lease, NetStatus};
pub use net_poll::net_poll_due;
pub use net_status::net_status;
pub use nonos_wifi_client::{ConnectResult, DriverStage, ScanNetwork, ScanOutcome, ScanStats};
