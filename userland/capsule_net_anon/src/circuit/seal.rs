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

//! Wrapping an outbound payload in one layer per hop.

use crate::cell::{put_digest, PAYLOAD_BYTES};

use super::hop::Hop;

/*
 * Digest first, over the cleartext with the integrity field zero, then the
 * keystreams from the target hop outward so the guard's layer ends up
 * outermost. circuit_package_relay_cell walks the cpath the same way, farthest
 * hop first.
 *
 * Takes the whole path, not one hop: the target's running digest has to stay in
 * step with the relay's copy, and a cell that skipped it desynchronises the
 * chain permanently.
 */
pub fn seal(hops: &mut [Hop], target: usize, payload: &mut [u8; PAYLOAD_BYTES]) -> Option<()> {
    let hop = hops.get_mut(target)?;
    hop.forward_digest.update(&payload[..]);
    let digest = hop.forward_digest.peek();
    put_digest(payload, &digest);
    for index in (0..=target).rev() {
        hops[index].forward.apply(&mut payload[..]);
    }
    Some(())
}
