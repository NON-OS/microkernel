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

mod adapter_name;
mod adapters;
mod datapath;
mod datapath_ask;
mod driver_ids;
mod interface;
mod lease;
mod lease_read;
mod net_poll;
mod net_status;
pub mod station;
mod station_words;
mod station_ask;
mod wifi_class;

pub use adapters::scan_adapters;
pub use datapath::DataPath;
pub use datapath_ask::driver_datapath;
pub use driver_ids::has_driver;
pub use interface::WifiInterface;
pub use lease::{Lease, NetStatus};
pub use net_poll::net_poll_due;
pub use net_status::net_status;
pub use station_ask::station_source;
pub use nonos_wifi_client::{ConnectResult, DriverStage, ScanNetwork, ScanOutcome, ScanStats};
