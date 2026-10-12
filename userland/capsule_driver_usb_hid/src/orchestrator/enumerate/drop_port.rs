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

//! Which endpoints go when a device leaves, and what each lets go of first:
//! every key and button its interface held comes up and its key repeat
//! ends, before its state is dropped. Pure but for those posts, so the
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
    for &slot in &slots {
        drop_slot(eps, slot);
    }
    slots
}

/// Drop every endpoint bound on `slot`, each releasing what it held.
pub fn drop_slot(eps: &mut Vec<HidEndpoint>, slot: u8) {
    for ep in eps.iter_mut().filter(|e| e.slot == slot) {
        ep.dev.release_all();
    }
    eps.retain(|e| e.slot != slot);
}
