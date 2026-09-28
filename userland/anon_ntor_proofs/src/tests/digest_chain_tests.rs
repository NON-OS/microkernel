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

//! That the digest really is a chain, and breaks if a cell is skipped.

extern crate alloc;

use super::onion_fixture::{chain, message};
use crate::cell::take_digest;
use crate::circuit::seal;

#[test]
fn a_skipped_cell_desynchronises_every_cell_after_it() {
    let mut client = chain();
    let mut relays = chain();
    for round in 0..3u8 {
        let mut payload = message(2, 1, &[round; 8]);
        seal(&mut client, 0, &mut payload).expect("target hop exists");
        if round == 1 {
            // The guard never sees this one, so its digest does not advance.
            continue;
        }
        relays[0].forward.apply(&mut payload[..]);
        let mut trial = relays[0].forward_digest.clone();
        let carried = take_digest(&mut payload);
        trial.update(&payload[..]);
        let matched = carried == trial.peek()[..4];
        if round == 0 {
            assert!(matched, "the first cell should verify");
            relays[0].forward_digest = trial;
        } else {
            assert!(!matched, "a cell after a skip must not verify");
        }
    }
}
