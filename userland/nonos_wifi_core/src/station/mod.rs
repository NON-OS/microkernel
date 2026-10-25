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

//! The station data-path engine shared by every WiFi driver: a driver feeds
//! one `LinkStation` its association facts and converts between ethernet and
//! 802.11 with it. It owns the transmit sequence number and the software
//! packet-number counter, so no driver keeps its own.
//!
//! [`LinkStation::receive`] checks each frame against the association before
//! it reaches the stack and answers the AP's key handshakes (`eapol`);
//! `rx_frame` remains the bare structural decap. `receive_mgmt` takes the
//! management frames, under management frame protection when it is on, and
//! `receive` hands them there too; `tick` runs the station's clock: the SA
//! Query it asked, and the probes after an access point gone quiet.

mod address;
mod alive;
mod ask;
mod beacon;
mod ccmp_mode;
mod chip_keys;
mod chip_rx;
mod cipher;
mod eapol;
mod eapol_tx;
mod engine;
#[cfg(test)]
mod forget_tests;
mod heard;
mod keys;
mod loss;
mod mgmt;
mod mgmt_leave;
mod mgmt_protected;
mod mgmt_route;
mod mgmt_rx;
mod pmf_state;
mod receive;
mod rekey;
mod replay;
mod sa_query;
mod session;
mod tick;
mod tx;
mod view;
mod watch;
mod wipe;
pub use crate::pmf::{SA_QUERY_MAX_MS, SA_QUERY_RETRY_MS};
pub use alive::{PROBES, PROBE_EVERY_MS, QUIET_MS};
pub use beacon::BeaconRx;
pub use ccmp_mode::Ccmp;
pub use engine::LinkStation;
pub use loss::{Loss, Tick};
pub use mgmt_rx::{MgmtDrop, MgmtRx, SaTick};
pub use receive::{Rx, RxDrop};
pub use watch::SSID_MAX;
