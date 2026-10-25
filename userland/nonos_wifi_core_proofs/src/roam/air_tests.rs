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

//! Roaming on the simulated air: a station whose AP is heard weak looks,
//! only listening, and moves by reassociation to an AP of its network heard
//! far stronger. It tells its old AP it is away before each trip and back
//! after it.

use super::two_aps::{run_until, sent, two_aps, AA, BB};

#[test]
fn a_weak_ap_gives_way_to_a_much_stronger_one_of_the_network() {
    let mut st = two_aps(|_| {}, [-85, -50]);
    assert!(run_until(&mut st, |s| s.session().is_some_and(|s| s.bssid == BB)), "moved");
    let s = st.session().unwrap();
    assert_eq!(s.channel, 6);
    let reassoc = sent(&st, 0x20);
    assert_eq!(reassoc.len(), 1, "one Reassociation Request");
    assert_eq!(reassoc[0][4..10], BB);
    assert_eq!(reassoc[0][24 + 4..24 + 10], AA, "naming the AP left");
    assert!(st.radio().unwrap().ap2.as_ref().unwrap().tk.is_some(), "the new AP keyed the link");
    let nulls = sent(&st, 0x48);
    assert!(nulls.iter().any(|f| f[1] == 0x11 && f[4..10] == AA), "away");
    assert!(nulls.iter().any(|f| f[1] == 0x01 && f[4..10] == AA), "and back");
    assert!(sent(&st, 0x40).is_empty(), "no probe request: the look only listens");
}

#[test]
fn an_ap_not_past_the_margin_or_a_strong_one_heard_already_is_not_a_reason() {
    let mut st = two_aps(|_| {}, [-85, -80]);
    assert!(!run_until(&mut st, |s| s.session().is_some_and(|s| s.bssid != AA)));
    assert!(st.session().is_some_and(|s| s.bssid == AA), "still on the first AP");
    assert!(!sent(&st, 0x48).is_empty(), "it looked");
    let mut st = two_aps(|_| {}, [-60, -30]);
    assert!(!run_until(&mut st, |s| s.session().is_some_and(|s| s.bssid != AA)));
    assert!(sent(&st, 0x48).is_empty(), "an AP heard well starts no look");
}
