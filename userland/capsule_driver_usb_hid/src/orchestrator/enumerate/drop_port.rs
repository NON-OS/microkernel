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

//! Which endpoints go when a device leaves its root port. Pure, so the
//! bookkeeping is proven on the host.

use alloc::vec::Vec;

use super::types::HidEndpoint;

/// Drop every endpoint bound on root port `port` and return the slots they
/// were on, each once, for the caller to give back.
pub fn drop_port(eps: &mut Vec<HidEndpoint>, port: u8) -> Vec<u8> {
    let mut slots = Vec::new();
    for ep in eps.iter().filter(|e| e.root_port == port) {
        if !slots.contains(&ep.slot) {
            slots.push(ep.slot);
        }
    }
    eps.retain(|e| e.root_port != port);
    slots
}
