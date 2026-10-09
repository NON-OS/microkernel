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

//! Programming the part once setup holds every grant: reset, station
//! address, PHY power, link, MAC enables, then the two queues.

mod announce;
pub mod control;
mod finish;
pub mod link;
pub mod mac_filter;
pub mod phy;
pub mod reset;
mod run;
pub mod rx_queue;
mod stand_down;
mod station_address;
pub mod tx_queue;
pub mod wait;

pub use finish::finish;
