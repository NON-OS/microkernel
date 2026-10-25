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

//! The claim over every order of entries by three CPUs, and the word's
//! packing of stage and owner.

use super::helpers::enter;
use super::stage::{word, Next, IDLE, STOPPING, WIPING};

#[test]
fn every_order_of_entries_has_one_full_and_wipes_only_for_its_owner() {
    // Each sequence of up to five entries by cpus 1 to 3.
    for code in 0..3u32.pow(5) {
        let mut s = IDLE;
        let mut full = None;
        for k in 0..5 {
            let me = (code / 3u32.pow(k) % 3 + 1) as u64;
            match enter(&mut s, me) {
                Next::Full => {
                    assert!(full.is_none(), "{code}: a second Full");
                    full = Some(me);
                }
                Next::Wipe => {
                    assert_eq!(full, Some(me), "{code}: a Wipe for a cpu that is not the owner")
                }
                Next::Park => assert!(
                    full.is_some_and(|o| o != me),
                    "{code}: the owner parked before its wipe"
                ),
            }
        }
    }
}

#[test]
fn the_word_keeps_the_stage_and_the_owner_apart() {
    for stage in [IDLE, STOPPING, WIPING] {
        for cpu in [1u64, 2, 255, 256, 0xFFFF_FFFF + 1] {
            let w = word(stage, cpu);
            assert_eq!((w & 0xFF, w >> 8), (stage, cpu));
        }
    }
}
