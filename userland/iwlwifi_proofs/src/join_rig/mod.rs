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

//! The rig the join proofs run on: the modeled device booted to ALIVE with
//! a transmit region, a firmware responder that answers the join's commands
//! (session protection with its notification, as the firmware sends it),
//! and a scripted access point that runs Open System or SAE authentication,
//! association, the four-way handshake and, once keyed, CCMP data, built
//! from the same shared core the station runs (its SAE, key derivation and
//! the authenticator messages of `nonos_wifi_core_proofs`). Each refusal and
//! silence the proofs need is a switch on the access point or the responder.

mod ap;
mod ap_auth;
mod ap_data;
mod ap_keys;
mod ap_out;
mod ap_sae;
mod ap_tx;
mod consts;
mod frames;
mod rig;
mod rig_reads;
mod session;
mod setup;
mod station;

pub use ap::*;
pub use consts::*;
pub use frames::*;
pub use rig::*;
pub use rig_reads::*;
pub use session::*;
pub use setup::*;
pub use station::*;
