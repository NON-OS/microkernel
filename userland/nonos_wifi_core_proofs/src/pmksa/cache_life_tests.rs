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

//! The PMKSA cache forgets on its own and when told: an entry outlives the
//! PMK lifetime by not a millisecond, and one entry or every entry goes
//! when the caller says so.

use super::cache_tests::{cache, sa, tag, PASS, SSID, STA};
use nonos_wifi_core::pmksa::PMK_LIFETIME_MS;

#[test]
fn an_entry_lives_for_the_pmk_lifetime_and_no_longer() {
    let mut c = cache();
    let t = tag(&c, SSID, PASS);
    c.keep(STA, SSID, &t, sa(1), 1_000);
    assert!(c.find((sa(1).bssid, STA), SSID, &t, 1_000 + PMK_LIFETIME_MS - 1).is_some());
    assert!(c.find((sa(1).bssid, STA), SSID, &t, 1_000 + PMK_LIFETIME_MS).is_none());
    assert!(c.is_empty(), "an outlived entry is removed, not kept unoffered");
}

#[test]
fn an_entry_or_everything_is_forgotten_when_told() {
    let mut c = cache();
    let t = tag(&c, SSID, PASS);
    (1..=3).for_each(|n| c.keep(STA, SSID, &t, sa(n), 0));
    c.remove(&sa(2).bssid, SSID);
    assert_eq!(c.len(), 2);
    assert!(c.find((sa(2).bssid, STA), SSID, &t, 1).is_none());
    c.wipe_all();
    assert!(c.is_empty());
    c.keep(STA, &[b'x'; 33], &t, sa(4), 0);
    assert!(c.is_empty(), "no SSID is longer than 32 octets");
}
