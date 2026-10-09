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

//! Peeling an inbound payload and working out whose it was.

use crate::cell::{put_digest, take_digest, unpack, PAYLOAD_BYTES};
use crate::crypto::equal;

use super::hop::Hop;

pub struct Opened {
    pub hop: usize,
}

/*
 * Both tests have to hold. `recognized` alone is two zero bytes, which a wrong
 * guess hits once in 65536 cells, so the digest is checked as well.
 *
 * A hop that does not match must leave its digest untouched, or the next cell
 * that really is from it will not verify. relay_digest_matches does the same by
 * checkpointing; here the trial runs on a clone and is only committed on a
 * match. The zeroed integrity field is restored too.
 */
pub fn open(hops: &mut [Hop], payload: &mut [u8; PAYLOAD_BYTES]) -> Option<Opened> {
    for (index, hop) in hops.iter_mut().enumerate() {
        hop.backward.apply(&mut payload[..]);
        if unpack(payload).recognized != 0 {
            continue;
        }
        let carried = take_digest(payload);
        let mut trial = hop.backward_digest.clone();
        trial.update(&payload[..]);
        let expected = trial.peek();
        if equal(&carried, &expected[..4]) {
            hop.backward_digest = trial;
            hop.last_seen = expected;
            return Some(Opened { hop: index });
        }
        put_digest(payload, &carried);
    }
    None
}
