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

//! The PMKSA cache: an entry is found only for its own AP, network and
//! passphrase, a newer one for the same AP replaces it, and the cache never
//! holds more than `PMKSA_MAX` entries, giving up the oldest.

use nonos_wifi_core::pmksa::{Pmksa, PmksaCache, PMKSA_MAX};

pub(super) const SSID: &[u8] = b"home";
pub(super) const PASS: &[u8] = b"correct horse";

pub(super) fn sa(n: u8) -> Pmksa {
    Pmksa { bssid: [2, 0, 0, 0, 0, n], pmk: [n; 32], pmkid: [n ^ 0x55; 16] }
}

#[test]
fn an_entry_is_found_only_for_its_ap_network_and_passphrase() {
    let mut c = PmksaCache::default();
    c.keep(SSID, PASS, sa(1), 0);
    assert_eq!(c.find(&sa(1).bssid, SSID, PASS, 1), Some(sa(1)));
    assert_eq!(c.find(&sa(2).bssid, SSID, PASS, 1), None, "another AP");
    assert_eq!(c.find(&sa(1).bssid, b"work", PASS, 1), None, "another network");
    assert_eq!(c.find(&sa(1).bssid, SSID, b"Correct horse", 1), None, "another passphrase");
    assert_eq!(c.find(&sa(1).bssid, b"hom", PASS, 1), None, "a prefix of the name");
}

#[test]
fn a_new_pmksa_for_the_same_ap_replaces_the_old() {
    let mut c = PmksaCache::default();
    c.keep(SSID, PASS, sa(1), 0);
    let newer = Pmksa { pmk: [9; 32], ..sa(1) };
    c.keep(SSID, PASS, newer, 5);
    assert_eq!(c.len(), 1);
    assert_eq!(c.find(&sa(1).bssid, SSID, PASS, 6), Some(newer));
}

#[test]
fn a_full_cache_gives_up_its_oldest_entry() {
    let mut c = PmksaCache::default();
    for n in 0..PMKSA_MAX as u8 {
        c.keep(SSID, PASS, sa(n), u64::from(n) * 10);
    }
    // Renewed, the first entry is the newest; the second is now the oldest.
    c.keep(SSID, PASS, sa(0), 50);
    c.keep(SSID, PASS, sa(9), 60);
    assert_eq!(c.len(), PMKSA_MAX);
    assert_eq!(c.find(&sa(1).bssid, SSID, PASS, 100), None, "the oldest went");
    assert!([0, 2, 3, 9].iter().all(|&n| c.find(&sa(n).bssid, SSID, PASS, 100).is_some()));
}
