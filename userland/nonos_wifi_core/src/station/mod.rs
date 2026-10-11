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

//! The station data-path engine shared by every WiFi driver. A driver holds one
//! `LinkStation`, feeds it its association facts, and then converts between
//! ethernet and 802.11 with two calls; its `LinkPort` implementation becomes a
//! thin shim over its own rings.
//!
//! The engine keeps the difference between a hardware-CCMP chip (the radio
//! encrypts from its key cache, so the station only frames) and a software-CCMP
//! chip (the station runs AES-CCM itself) in one place, so no driver picks the
//! wrong path and double-encrypts. It owns the transmit sequence number and, for
//! software CCMP, the packet-number counter, so those are never duplicated per
//! driver either.
//!
//! Receive goes through [`LinkStation::receive`] (`receive`), which checks each
//! frame against the association (BSS, protection, replay) before it reaches
//! the stack, and answers the access point's key handshakes with the
//! supplicant the join finished with (`eapol`). `rx_frame` remains the bare
//! structural decap, without those checks.

mod eapol;
mod frames;
mod keys;
mod link;
mod mode;
mod receive;
mod replay;
mod session;

pub use link::LinkStation;
pub use mode::Ccmp;
pub use receive::{Rx, RxDrop};
