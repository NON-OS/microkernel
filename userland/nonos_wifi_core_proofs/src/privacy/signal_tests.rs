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

//! Row 1.7: the current AP's signal is judged only from frames that verify
//! under the session's keys. Beacons prove nothing, so a run of weak ones,
//! forged or not, starts no look; weak protected traffic does.

use crate::roam::two_aps::{run_until, sent, two_aps};

#[test]
fn weak_beacons_alone_start_no_look() {
    let mut st = two_aps(|_| {}, [-85, -40]);
    st.radio_mut().unwrap().ap.chatter = false;
    assert!(!run_until(&mut st, |s| !sent(s, 0x48).is_empty()), "never away");
    assert!(sent(&st, 0x20).is_empty());
}

#[test]
fn weak_protected_traffic_does() {
    let mut st = two_aps(|_| {}, [-85, -40]);
    assert!(run_until(&mut st, |s| !sent(s, 0x48).is_empty()), "it looked");
}
