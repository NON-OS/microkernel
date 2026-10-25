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

use super::boot_claim::BootClaim;
use std::sync::Arc;

#[test]
fn only_an_entered_ap_is_asked_to_park_and_only_once() {
    let c = BootClaim::new();
    c.arm();
    assert!(!c.ask_park(), "an AP that never entered is abandoned, not asked");
    assert!(!c.ap_park(), "nothing to answer");
    assert!(c.ap_enter());
    assert!(!c.park_asked() && !c.parked());
    assert!(c.ask_park());
    assert!(c.park_asked() && c.entered());
    assert!(!c.ask_park(), "asked once");
    assert!(!c.bsp_abandon(), "it entered; it is not abandoned");
    assert!(c.ap_park());
    assert!(c.parked() && !c.park_asked() && c.entered());
    assert!(!c.ap_park(), "parked once");
    assert!(!c.ap_enter(), "a parked AP never enters again");

    let c = BootClaim::new();
    c.arm();
    assert!(c.bsp_abandon());
    assert!(!c.ask_park() && !c.ap_park() && !c.parked());
}

#[test]
fn the_park_race_has_one_answer() {
    // The boot CPU asks while the AP checks: the AP either sees the request
    // and parks, or passes its last checkpoint first and is never parked.
    for _ in 0..2000 {
        let c = Arc::new(BootClaim::new());
        c.arm();
        assert!(c.ap_enter());
        let c2 = c.clone();
        let ap = std::thread::spawn(move || c2.park_asked() && c2.ap_park());
        let asked = c.ask_park();
        let parked = ap.join().unwrap();
        assert!(asked);
        assert_eq!(parked, c.parked());
        assert_eq!(!parked, c.park_asked());
    }
}
