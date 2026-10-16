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

use super::cache_tests::{sa, PASS, SSID};
use nonos_wifi_core::pmksa::{PmksaCache, PMK_LIFETIME_MS};

#[test]
fn an_entry_lives_for_the_pmk_lifetime_and_no_longer() {
    let mut c = PmksaCache::default();
    c.keep(SSID, PASS, sa(1), 1_000);
    assert!(c.find(&sa(1).bssid, SSID, PASS, 1_000 + PMK_LIFETIME_MS - 1).is_some());
    assert_eq!(c.find(&sa(1).bssid, SSID, PASS, 1_000 + PMK_LIFETIME_MS), None);
    assert!(c.is_empty(), "an outlived entry is removed, not kept unoffered");
}

#[test]
fn an_entry_or_everything_is_forgotten_when_told() {
    let mut c = PmksaCache::default();
    (1..=3).for_each(|n| c.keep(SSID, PASS, sa(n), 0));
    c.remove(&sa(2).bssid, SSID);
    assert_eq!(c.len(), 2);
    assert_eq!(c.find(&sa(2).bssid, SSID, PASS, 1), None);
    c.wipe_all();
    assert!(c.is_empty());
    c.keep(&[b'x'; 33], PASS, sa(4), 0);
    assert!(c.is_empty(), "no SSID is longer than 32 octets");
}
