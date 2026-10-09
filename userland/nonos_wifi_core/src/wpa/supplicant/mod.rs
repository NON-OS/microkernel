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

//! The RSN 4-way and group key handshakes, supplicant side, for WPA2-PSK,
//! PSK-SHA256 and WPA3-SAE. This is chip-independent: it consumes only the
//! proven key derivation (`wpa::ptk`, `wpa::akm`), the EAPOL codec (`eapol`),
//! and the AES key-unwrap (`ccmp::keywrap`), so any WiFi driver (Intel,
//! Realtek) drives the same state machine. Given the PMK, the two MAC
//! addresses and the elements of the association, `step` is fed each
//! EAPOL-Key frame the AP sends and returns the frame to transmit back,
//! deriving the pairwise key on message 1, taking the group keys on message 3
//! and on each group key handshake after it.

mod group;
pub mod ie;
mod keys;
mod message3;
mod reply;
mod state;
pub mod step;

pub use state::{Config, Failure, State, Supplicant};
pub use step::StepOutput;
