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

//! A hostile keyboard stream through the driver's decode: every key event
//! names a defined key, presses only a key that is up and releases only
//! one that is down, and the keys down are the last whole report's.

use super::key_report::{hostile_key_report, is_key};
use super::ROUNDS;
use crate::keyboard_tests::step;
use crate::rng::Rng;
use std::collections::BTreeSet;

#[test]
fn a_hostile_keyboard_stream_presses_and_releases_exactly_what_its_reports_imply() {
    let mut rng = Rng::seeded(0x5eed_0003);
    let (mut held, mut down) = ([0u8; 6], BTreeSet::new());
    let mut keys = [0u8; 8];
    for round in 0..ROUNDS {
        if round % 4096 == 0 {
            keys.iter_mut().for_each(|k| *k = rng.byte());
            keys[0] = 0x04;
        }
        let raw = hostile_key_report(&mut rng, &keys);
        let (events, next, mods) = step(held, &raw);
        // Every event names a key and agrees with the stream so far: a
        // press only of a key up, a release only of a key down.
        for &(key, pressed) in &events {
            assert!(is_key(key), "round {round}: event for usage {key:#x}");
            let flipped = if pressed { down.insert(key) } else { down.remove(&key) };
            assert!(flipped, "round {round}: {key:#x} pressed {pressed} twice in a row");
        }
        let whole = raw.len() == 8;
        let says_nothing = !whole || raw[2..].iter().any(|k| (1..=3).contains(k));
        if says_nothing {
            assert!(events.is_empty(), "round {round}: {raw:02x?} made {events:?}");
            assert_eq!(next, held, "round {round}");
        } else {
            let named: BTreeSet<u8> = raw[2..].iter().copied().filter(|&k| is_key(k)).collect();
            assert_eq!(down, named, "round {round}: {raw:02x?}");
        }
        assert_eq!(mods, whole.then(|| raw[0]), "round {round}");
        let still: BTreeSet<u8> = next.iter().copied().filter(|&k| is_key(k)).collect();
        assert_eq!(still, down, "round {round}");
        held = next;
    }
}
