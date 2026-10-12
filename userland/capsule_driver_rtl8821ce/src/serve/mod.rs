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

//! The serving stage. Once the firmware is running and the MAC is initialised,
//! this builds the radio and answers requests forever. Two request families
//! arrive on the one service inbox: net_core's link operations, handled by the
//! shared `netif::serve` against the `RtlLink`, and a Wi-Fi control family, handled
//! by [`control`], that scans and joins. The loop also advances a background
//! scanner between requests so a scan is answered from a cache rather than a sweep.
//!
//! The stage is split by concern: [`stage`] is the bring-up result, [`radio`]
//! builds and holds the radio, [`scanner`] keeps the network picture current,
//! [`control`] is the control protocol, and [`connect`] runs a WPA2 join.

mod after;
mod answer;
mod connect;
mod control;
mod loss;
mod net_core;
mod radio;
mod refuse;
mod run;
mod scanner;
mod stage;
mod tuning;

pub use run::run;
pub use stage::Stage;
use tuning::{DEFAULT_CHANNEL, SCAN_CHANNELS, SCAN_FRAME_MAX};
