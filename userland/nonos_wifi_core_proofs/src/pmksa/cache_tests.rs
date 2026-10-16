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

//! The PMKSA cache: an entry is found only for its own AP, network,
//! station address and passphrase, a newer one for the same AP replaces
//! it, and the cache never holds more than `PMKSA_MAX` entries, giving up
//! the oldest.

pub use super::fixture::{cache, sa, tag, twin, PASS, SSID, STA};
use nonos_wifi_core::pmksa::{Pmksa, PMKSA_MAX};

#[test]
fn an_entry_is_found_only_for_its_ap_network_and_passphrase() {
    let mut c = cache();
    let t = tag(&c, SSID, PASS);
    c.keep(STA, SSID, &t, sa(1), 0);
    assert!(c.find((sa(1).bssid, STA), SSID, &t, 1) == Some(sa(1)));
    assert!(c.find((sa(2).bssid, STA), SSID, &t, 1).is_none(), "another AP");
    assert!(c.find((sa(1).bssid, STA), b"work", &t, 1).is_none(), "another network");
    let other = tag(&c, SSID, b"Correct horse");
    assert!(c.find((sa(1).bssid, STA), SSID, &other, 1).is_none(), "another passphrase");
    assert!(c.find((sa(1).bssid, STA), b"hom", &t, 1).is_none(), "a prefix of the name");
    assert!(c.find((sa(1).bssid, [2, 0, 0, 0, 0, 1]), SSID, &t, 1).is_none(), "another station");
}

#[test]
fn a_new_pmksa_for_the_same_ap_replaces_the_old() {
    let mut c = cache();
    let t = tag(&c, SSID, PASS);
    c.keep(STA, SSID, &t, sa(1), 0);
    let newer = Pmksa { pmk: [9; 32], ..sa(1) };
    c.keep(STA, SSID, &t, twin(&newer), 5);
    assert_eq!(c.len(), 1);
    assert!(c.find((sa(1).bssid, STA), SSID, &t, 6) == Some(newer));
}

#[test]
fn a_full_cache_gives_up_its_oldest_entry() {
    let mut c = cache();
    let t = tag(&c, SSID, PASS);
    for n in 0..PMKSA_MAX as u8 {
        c.keep(STA, SSID, &t, sa(n), u64::from(n) * 10);
    }
    // Renewed, the first entry is the newest; the second is now the oldest.
    c.keep(STA, SSID, &t, sa(0), 50);
    c.keep(STA, SSID, &t, sa(9), 60);
    assert_eq!(c.len(), PMKSA_MAX);
    assert!(c.find((sa(1).bssid, STA), SSID, &t, 100).is_none(), "the oldest went");
    assert!([0, 2, 3, 9].iter().all(|&n| c.find((sa(n).bssid, STA), SSID, &t, 100).is_some()));
}
