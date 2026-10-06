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

//! Reporting how far the transport has got.

use crate::manager::{Bootstrap, Manager};
use crate::protocol::HDR_LEN;

/// Body: bootstrap stage, relay count, circuit count, open circuit count.
///
pub fn status(state: &Manager, tx: &mut [u8]) -> u32 {
    let stage: u8 = match state.bootstrap {
        Bootstrap::Cold => 0,
        Bootstrap::Anchored => 1,
        Bootstrap::Joining => 2,
        Bootstrap::Ready => 3,
    };
    let open =
        state.circuits.iter().filter(|c| c.stage == crate::circuit::CircuitStage::Open).count();
    let body = HDR_LEN;
    tx[body] = stage;
    tx[body + 1..body + 5].copy_from_slice(&(state.relays.len() as u32).to_le_bytes());
    tx[body + 5..body + 9].copy_from_slice(&(state.circuits.len() as u32).to_le_bytes());
    tx[body + 9..body + 13].copy_from_slice(&(open as u32).to_le_bytes());
    13
}
