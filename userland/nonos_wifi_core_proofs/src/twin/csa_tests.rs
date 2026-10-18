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

//! Channel switch. The station acts on no Channel Switch Announcement: not
//! in a beacon, not in a Channel Switch action frame, protected or not. A
//! forged one therefore moves nothing; it costs the station nothing but the
//! frame. The AP's own switch is not followed either: its beacons stop on
//! the old channel, the link ends when they are missed, and the station
//! joins again by scanning, wherever the AP went.

use super::watch::put;
use crate::soft::join::{joined, sta_of, ticks};

/// The Channel Switch Announcement element (802.11-2020, 9.4.2.18,
/// recalled): mode 1 (stop sending), channel 11, in 2 beacons.
const CSA: [u8; 5] = [37, 3, 1, 11, 2];

#[test]
fn a_channel_switch_announcement_moves_nothing() {
    let mut st = joined();
    let tuned = st.radio().unwrap().tuned;
    let link = st.session().map(|s| (s.mac(), s.bssid));
    // In the AP's beacon.
    let beacon = [st.radio_mut().unwrap().ap.beacon_out(), CSA.to_vec()].concat();
    put(&mut st, |_| beacon);
    // In a Spectrum Management Channel Switch action frame (category 0,
    // action 4), unprotected.
    let action = [&[0u8, 4][..], &CSA].concat();
    put(&mut st, |s| crate::soft::frames::mgmt(13, sta_of(s), &action));
    ticks(&mut st, 300);
    assert_eq!(st.radio().unwrap().tuned, tuned, "still on channel 6");
    assert_eq!(st.session().map(|s| (s.mac(), s.bssid)), link, "the link held");
}
