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

//! Building the hostile control request each round shares.

use crate::sweep::rng::Rng;
use nonos_wifi_core::netif::wire::MAGIC_NNET;
use nonos_wifi_core::softmac::engine::WIFI_MAGIC;

/// A request with a chosen magic, op and a random tail, sometimes cut short
/// of its header. `ops` is the pool the op is drawn from.
pub(super) fn request(rng: &mut Rng, ops: &[u16]) -> Vec<u8> {
    let junk = rng.next() as u32;
    let magic = rng.pick(&[WIFI_MAGIC, MAGIC_NNET, 0xDEAD_BEEF, junk]);
    let mut r = magic.to_le_bytes().to_vec();
    r.extend_from_slice(&rng.pick(ops).to_le_bytes());
    r.extend(rng.span(0, 48)); // the rest of a header, then a body, or junk
    if rng.byte() & 7 == 0 {
        r.truncate(rng.below(r.len() + 1)); // sometimes shorter than a header
    }
    r
}
