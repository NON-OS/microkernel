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

//! A look on its own: it keeps only beacons of the session's network, the
//! strongest per AP and at most `HEARD_MAX`; never visits its own channel;
//! sends nothing but the away and back Null frames; and ends home, with
//! the filter on its AP again.

use super::beacons::{ap, beacon_of, RSNE_PSK};
use super::bench::Bench;
use nonos_wifi_core::regdom::{Band, WORLD};
use nonos_wifi_core::softmac::roam::{Look, Step, HEARD_MAX};
use nonos_wifi_core::softmac::Radio;

const HOME: [u8; 6] = [2, 0, 0, 0, 0, 1];

fn look_with(bench: &mut Bench) -> Look {
    let mut look = Look::new(WORLD, bench.bands(), (Band::Ghz2, 1), bench.mac(), HOME);
    let mut now = 0;
    while look.step(bench, b"home", now, false) != Step::Done {
        now += 5;
        assert!(now < 100_000, "a look ends");
    }
    look
}

#[test]
fn only_the_networks_aps_are_kept_and_each_at_its_strongest() {
    let mut b = Bench::default();
    let mine = beacon_of(ap(6), b"home", 6, &RSNE_PSK);
    let other = beacon_of(ap(7), b"work", 6, &RSNE_PSK);
    b.waiting.insert(6, vec![(mine.clone(), -70), (other, -30), (mine, -60)]);
    let look = look_with(&mut b);
    assert_eq!(look.heard.len(), 1);
    assert_eq!(
        (look.heard[0].bssid, look.heard[0].rssi_dbm, look.heard[0].channel),
        (ap(6), -60, 6)
    );
}

#[test]
fn at_most_heard_max_aps_are_kept() {
    let mut b = Bench::default();
    let many =
        (0..HEARD_MAX as u8 + 5).map(|n| (beacon_of(ap(n), b"home", 11, &RSNE_PSK), -50)).collect();
    b.waiting.insert(11, many);
    assert_eq!(look_with(&mut b).heard.len(), HEARD_MAX);
}

#[test]
fn the_look_visits_every_other_channel_and_ends_home_having_sent_only_nulls() {
    let mut b = Bench::default();
    look_with(&mut b);
    let away: Vec<u8> = b.tuned.iter().copied().filter(|&c| c != 1).collect();
    assert_eq!(away, (2..=13).collect::<Vec<u8>>(), "the world domain's 2.4 GHz channels but home");
    assert_eq!((b.tuned.last(), b.bss), (Some(&1), Some(HOME)), "home, filter on its AP");
    let trips = b.sent.len() / 2;
    assert_eq!(b.tuned.iter().filter(|&&c| c == 1).count(), trips, "home only to come back");
    assert!(b.sent.iter().all(|f| f[0] == 0x48), "only Null frames");
    let flags: Vec<u8> = b.sent.iter().map(|f| f[1]).collect();
    assert!(flags.chunks(2).all(|p| p == [0x11, 0x01]), "each trip away, then back");
}
