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

//! How weak the session's AP is heard: an average a run of beacons moves,
//! that counts only after `SAMPLES_MIN`; and the frame that tells the AP
//! the station is away, and back, laid out by hand (802.11-2020, 11.2.3).

use nonos_wifi_core::softmac::roam::{pm_null, Signal, SAMPLES_MIN, WEAK_DBM};

#[test]
fn the_ap_is_weak_only_after_a_run_of_weak_beacons() {
    let mut s = Signal::default();
    for _ in 0..SAMPLES_MIN - 1 {
        s.heard(-90);
    }
    assert!(!s.weak(), "too few to count");
    s.heard(-90);
    assert!(s.weak());
    assert_eq!(s.avg_dbm(), -90);
    s.heard(-50);
    assert_eq!(s.avg_dbm(), -85, "an eighth of the way");
    for _ in 0..40 {
        s.heard(-50);
    }
    assert!(!s.weak() && s.avg_dbm() > WEAK_DBM);
    s.reset();
    assert!(!s.weak());
}

#[test]
fn the_null_frames_say_away_then_back() {
    let (sta, ap) = ([2, 0, 0, 0, 0, 2], [2, 0, 0, 0, 0, 1]);
    let away = pm_null(sta, ap, true);
    assert_eq!(away[..2], [0x48, 0x11], "data, Null; To DS and Power Management");
    assert_eq!((&away[4..10], &away[10..16], &away[16..22]), (&ap[..], &sta[..], &ap[..]));
    assert_eq!(pm_null(sta, ap, false)[..2], [0x48, 0x01], "To DS only");
}
