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

//! Programming the part once setup holds every grant: reset, link mode,
//! station address, filters, MAC settings, then the two queues and the
//! receiver.

mod announce;
pub mod autoc;
pub mod control;
mod finish;
mod hw_control;
pub mod mac_filter;
pub mod reset;
mod run;
pub mod rx_disable;
mod rx_dma;
pub mod rx_queue;
mod stand_down;
mod station_address;
pub mod tx_queue;
pub mod wait;

pub use finish::finish;
