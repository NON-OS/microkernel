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

//! When a session may look: only once its AP is heard weak, and never
//! again within `COOLDOWN_MS` of the last look or move.

use super::beacons::{ap, beacon_of, RSNE_PSK};
use nonos_wifi_core::rsn::{Ciphers, JoinPolicy};
use nonos_wifi_core::softmac::roam::{offered, Kit, COOLDOWN_MS, SAMPLES_MIN};

#[test]
fn a_look_is_due_only_for_a_weak_ap_and_not_within_the_cooldown() {
    let runs = offered(&beacon_of(ap(1), b"home", 1, &RSNE_PSK), JoinPolicy::PSK_ONLY).unwrap();
    let mut k = Kit::new(b"pass", false, runs, Ciphers::CCMP_128_ONLY, 0).unwrap();
    assert!(!k.due(0), "nothing heard yet");
    (0..SAMPLES_MIN).for_each(|_| k.signal.heard(-90));
    assert!(k.due(0));
    k.rest(1_000);
    assert!(!k.due(1_000 + COOLDOWN_MS - 1));
    assert!(k.due(1_000 + COOLDOWN_MS));
    assert!(Kit::new(&[b'x'; 65], false, runs, Ciphers::CCMP_128_ONLY, 0).is_none(), "too long");
}
