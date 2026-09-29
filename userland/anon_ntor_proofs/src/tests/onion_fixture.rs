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

//! A client and a relay chain keyed alike, for the layering proofs.

extern crate alloc;

use alloc::vec::Vec;

use crate::cell::{pack, RelayHeader, PAYLOAD_BYTES};
use crate::circuit::Hop;
use crate::ntor::KEY_MATERIAL_BYTES;

pub const HOPS: usize = 3;

/*
 * The simulated relays run the same `Hop` the client does. That is sound
 * because counter mode is its own inverse and a relay seeds its digest from the
 * same key material. What the proofs check is the layering order, the digest
 * placement and the recognition rule, all of which are ours to get wrong.
 */
/// Distinct key material per hop, so a layer applied at the wrong index
/// produces different bytes rather than accidentally cancelling out.
pub fn keys(hop: u8) -> [u8; KEY_MATERIAL_BYTES] {
    let mut out = [0u8; KEY_MATERIAL_BYTES];
    for (index, byte) in out.iter_mut().enumerate() {
        *byte = (index as u8).wrapping_mul(3).wrapping_add(hop.wrapping_mul(97)).wrapping_add(1);
    }
    out
}

/// Three hops keyed from `keys`, guard first.
pub fn chain() -> Vec<Hop> {
    (0..HOPS as u8).map(|hop| Hop::new(&keys(hop))).collect()
}

/// A relay message addressed to us, ready to be sealed.
pub fn message(command: u8, stream: u16, text: &[u8]) -> [u8; PAYLOAD_BYTES] {
    let header = RelayHeader { command, recognized: 0, stream, length: text.len() as u16 };
    pack(&header, text).expect("body fits a cell")
}
