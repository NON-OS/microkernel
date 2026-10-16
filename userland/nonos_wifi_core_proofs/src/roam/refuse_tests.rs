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

//! A move that does not go through: to an AP of other security none is
//! tried; one the new AP refuses leaves the station on its old AP, whose
//! association it never lost; and one the new AP accepted, but whose
//! handshake failed, ends the session.

use super::two_aps::{run_until, sent, two_aps, AA};
use crate::roam::beacons::RSNE_SAE;

#[test]
fn an_ap_of_other_security_is_never_tried() {
    let mut st = two_aps(|b| b.rsne = RSNE_SAE.to_vec(), [-85, -40]);
    assert!(!run_until(&mut st, |s| s.session().is_some_and(|s| s.bssid != AA)));
    assert!(sent(&st, 0x20).is_empty(), "no reassociation");
    assert!(st.session().is_some_and(|s| s.bssid == AA));
}

#[test]
fn a_refused_move_leaves_the_station_on_its_old_ap() {
    let mut st = two_aps(|b| b.refuse = true, [-85, -40]);
    assert!(run_until(&mut st, |st| !sent(st, 0x20).is_empty()), "it tried");
    let s = st.session().expect("the session stays up");
    assert_eq!((s.bssid, s.channel), (AA, 1));
    let air = st.radio().unwrap();
    assert_eq!((air.tuned.map(|t| t.1), air.bss), (Some(1), Some(AA)), "home again");
}

#[test]
fn a_move_whose_handshake_failed_ends_the_session() {
    let mut st = two_aps(|b| b.pass = b"another password".to_vec(), [-85, -40]);
    assert!(run_until(&mut st, |st| !sent(st, 0x20).is_empty() && st.session().is_none()));
    assert_eq!(st.radio().unwrap().bss, None, "the filter open again for the scan");
}
