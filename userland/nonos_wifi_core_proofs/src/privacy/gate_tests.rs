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

//! Rows 1.10 and 1.11: the link is busy, and no look leaves it, while an SA
//! Query awaits the AP's answer or a pairwise rekey is between message 1
//! and its new keys; a look held at home never goes.

use crate::ap_sim::{message1, AA, ANONCE, SNONCE, SPA};
use crate::pmf_sim::joined_pmf;
use crate::roam::bench::Bench;
use nonos_wifi_core::regdom::{Band, WORLD};
use nonos_wifi_core::softmac::roam::{Look, Step};
use nonos_wifi_core::softmac::Radio;
use nonos_wifi_core::station::{Ccmp, LinkStation};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::supplicant::Supplicant;

#[test]
fn an_sa_query_in_flight_makes_the_link_busy() {
    let (mut s, _) = joined_pmf();
    assert!(!s.busy());
    assert!(s.sa_query_start(0, [1, 2]).is_some());
    assert!(s.busy(), "until the AP answers");
}

#[test]
fn a_rekey_between_message_1_and_its_keys_makes_the_link_busy() {
    let mut sup = Supplicant::new(pmk(b"ThisIsAPassword", b"ThisIsASSID"), AA, SPA, SNONCE);
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let mut s = LinkStation::new(SPA);
    s.associate(AA, Ccmp::Software { tk: [0; 16] });
    s.set_supplicant(sup);
    assert!(s.busy());
}

#[test]
fn a_look_held_at_home_never_goes() {
    let mut bench = Bench::default();
    let mut look = Look::new(WORLD, bench.bands(), (Band::Ghz2, 1), bench.mac(), AA);
    for now in (0..20_000).step_by(5) {
        assert_eq!(look.step(&mut bench, b"home", now, true), Step::Home);
    }
    assert!(bench.tuned.is_empty() && bench.sent.is_empty(), "never tuned, nothing sent");
    assert_eq!(look.step(&mut bench, b"home", 20_000, false), Step::Away, "released, it goes");
}
