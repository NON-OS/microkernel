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

//! The Wi-Fi control family Settings, first-boot setup and net_core speak to
//! whichever Wi-Fi driver is running (`nonos_wifi_client`): tag 0x57494649,
//! `[magic u32][op u16][request id u32]`.
//!
//! On an SO platform (AX211, or AX201 on that platform) the firmware is
//! brought up and runs passive scans in the background; the status op then
//! reports Ready and the scan op answers at once from what was heard, in the
//! layout the RTL8821CE driver uses: three counters (sweeps completed, frames
//! received, beacons parsed) then the network list. When the radio can join
//! (it is up, a station address was drawn, and the firmware speaks every
//! join command's layout) connect, disconnect and link go to the radio
//! (`route`, `join_wire`); otherwise they are refused here with -38 rather
//! than left to time out. When the radio is not up the status op reports how
//! far bring-up got, and every other op is refused with -38.
//!
//! The same inbox also takes net_core's link protocol (tag "NNET",
//! `nonos_wifi_core::netif`), the one it speaks to the RTL8821CE.
//!
//! The status reply is the stage byte (all the client reads), then the step
//! bring-up stopped at, its detail word, CSR_HW_REV and CSR_HW_RF_ID, and the
//! scan counters (sweeps completed, sweeps stalled, frames, beacons), all
//! little-endian.

mod answer;
mod kinds;
mod route;

pub use answer::answer;
pub use kinds::{Heard, View, OP_CONNECT, OP_DISCONNECT, REPLY_MAX};
pub use route::{route, Route};
