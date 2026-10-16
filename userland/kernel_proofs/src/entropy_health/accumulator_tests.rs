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

//! The accumulator: nothing used before the source is proven, nothing kept
//! past an alarm, and never enough alone.

use super::accumulator::{Accumulator, Fed, HELD};
use super::bytes::uniform;
use super::credit::{enough, MAX_CREDIT, SEED_BYTES};
use super::health::{Fault, RCT_CUTOFF, STARTUP};

fn proven() -> Accumulator {
    let mut acc = Accumulator::new();
    for batch in uniform(10, STARTUP as usize).chunks(256) {
        assert_eq!(acc.feed(batch), Fed::Proving, "startup bytes are dropped");
    }
    assert_eq!(acc.take(), None, "nothing held from the startup");
    acc
}

#[test]
fn bytes_are_held_once_the_source_is_proven() {
    let mut acc = proven();
    assert!(acc.wanted());
    assert_eq!(acc.feed(&uniform(11, 256)), Fed::Held);
    assert!(!acc.wanted(), "full");
    let (held, counted) = acc.take().expect("held");
    assert_eq!(counted, HELD);
    assert_ne!(held, [0; HELD]);
    assert_eq!(acc.take(), None, "a take empties it");
    assert!(acc.wanted());
}

#[test]
fn an_alarm_refuses_the_batch_and_drops_what_is_held() {
    let mut acc = proven();
    assert_eq!(acc.feed(&uniform(12, 32)), Fed::Held);
    let stuck = vec![0xAA; RCT_CUTOFF as usize];
    assert_eq!(acc.feed(&stuck), Fed::Refused(Fault::Repetition));
    assert_eq!(acc.take(), None, "what was held went with the alarm");
    assert_eq!(acc.feed(&uniform(13, 256)), Fed::Proving, "proven again from the start");
}

#[test]
fn a_short_batch_counts_its_own_length() {
    let mut acc = proven();
    assert_eq!(acc.feed(&uniform(14, 8)), Fed::Held);
    assert_eq!(acc.take().map(|t| t.1), Some(8));
}

#[test]
fn the_capsule_is_never_enough_alone() {
    for capsule in 0..=4 * HELD {
        assert!(!enough(0, capsule), "{capsule} capsule bytes alone");
        assert!(!enough(SEED_BYTES - MAX_CREDIT - 1, capsule));
    }
    assert!(enough(SEED_BYTES - MAX_CREDIT, MAX_CREDIT));
    assert!(enough(SEED_BYTES, 0) && !enough(SEED_BYTES - 1, 0));
}
