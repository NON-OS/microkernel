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

//! Row 1.4: a roam's supplicant holds message 3 to the candidate's own
//! beacon elements, as a join's does. A candidate whose message 3 names
//! other security than its beacon fails the move as a downgrade.

use crate::roam::two_aps::{run_until, sent, two_aps};
use crate::soft::wire::control;
use nonos_wifi_core::softmac::control::roam_code::CODE_AP_UNPROVEN;

/// The status reply's word at 53: the last failed roam's code.
pub fn last_roam(st: &mut crate::soft::rig::Station) -> i32 {
    let s = control(st, 4, &[]);
    i32::from_le_bytes(s[53..57].try_into().unwrap())
}

/// The status reply's word at 57: the beacons beacon protection refused.
pub fn beacons_refused(st: &mut crate::soft::rig::Station) -> u32 {
    let s = control(st, 4, &[]);
    u32::from_le_bytes(s[57..61].try_into().unwrap())
}

#[test]
fn a_message_3_that_differs_from_the_candidates_beacon_fails_the_move() {
    // The beacon offers CCMP with PSK; message 3 claims TKIP for group.
    let other = vec![
        0x30, 0x14, 1, 0, 0, 0x0f, 0xac, 2, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 2, 0, 0,
    ];
    let mut st = two_aps(|b| b.quirks.msg3_rsne = Some(other), [-85, -40]);
    let tried = |s: &crate::soft::rig::Station| !sent(s, 0x20).is_empty() && s.session().is_none();
    assert!(run_until(&mut st, tried), "accepted, then refused: the session ends");
    assert!(st.radio().unwrap().ap2.as_ref().unwrap().tk.is_none(), "never keyed");
    assert_eq!(last_roam(&mut st), CODE_AP_UNPROVEN);
}
