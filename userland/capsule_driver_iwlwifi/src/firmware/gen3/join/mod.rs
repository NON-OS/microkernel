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

//! Joining a WPA2 or WPA3 network on the running firmware, and carrying data
//! once joined. The protocol and the keys are the shared core's
//! (`nonos_wifi_core`: the MLME, SAE, the supplicant, CCMP, the station data
//! path), as on the RTL8821CE; this is the firmware half:
//!
//! - `fw`: commands, the two transmit queues, and bounded waits, with
//!   everything the firmware posts routed through `inbox`;
//! - `hunt`: the network's beacon found by a passive sweep;
//! - `target`: the BSS as its beacon describes it;
//! - `bss`: the PHY, link, station, queue and session contexts, put up in
//!   Linux's order and taken down as far as they went;
//! - `exchange`: the MLME fed what is received and its frames sent, with
//!   retransmits and bounded waits;
//! - `keys`: the pairwise and group keys in the firmware's table;
//! - `link`: the open port and the `LinkPort` net_core drives;
//! - `run`: the whole join from a beacon, and leaving.

pub mod bss;
pub mod exchange;
pub mod fw;
pub mod hunt;
pub mod inbox;
pub mod keys;
pub mod link;
pub mod run;
pub mod target;
