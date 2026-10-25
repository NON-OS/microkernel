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

//! The BIGTK from message 3 and from a group key message.

use super::handshake::past_message1;
use super::sim::{joined_bp, kd3, signed};
use crate::ap_sim::{bigtk_kde, group1, gtk_kde, igtk_kde, message3, ANONCE};
use crate::pmf_sim::IGTK;
use crate::soft::protect::BIGTK;
use nonos_wifi_core::eapol::kde::parse_key_data;
use nonos_wifi_core::station::BeaconRx;
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::supplicant::State;

#[test]
fn the_bigtk_kde_is_read_like_the_igtks() {
    let kde = bigtk_kde(6, 3, &BIGTK);
    let k = parse_key_data(&kde).unwrap().bigtk.expect("a BIGTK");
    assert_eq!((k.key_id, k.ipn, k.key), (6, [3, 0, 0, 0, 0, 0], &BIGTK[..]));
    assert!(parse_key_data(&kde[..20]).is_none(), "a cut BIGTK KDE");
}

#[test]
fn message_3_delivers_it() {
    let (mut sup, ptk) = past_message1();
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd3(&bigtk_kde(6, 3, &BIGTK))));
    assert_eq!(sup.state(), State::Connected);
    assert_eq!(sup.bigtk(), Some((&BIGTK, 6, [3, 0, 0, 0, 0, 0])));
}

#[test]
fn a_bigtk_of_the_wrong_size_or_index_fails_the_handshake() {
    for kde in [bigtk_kde(4, 3, &BIGTK), [&bigtk_kde(6, 3, &BIGTK)[..], &[0; 8]].concat()] {
        let mut kde = kde;
        kde[1] = (kde.len() - 2) as u8;
        let (mut sup, ptk) = past_message1();
        sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd3(&kde)));
        assert_eq!(sup.state(), State::Failed);
        assert!(sup.bigtk().is_none());
    }
}

#[test]
fn a_rekey_brings_the_next_bigtk_and_its_own_floor() {
    let (mut s, ptk) = joined_bp();
    assert_eq!(s.receive_beacon(&signed(6, 50, &BIGTK)), BeaconRx::Verified);
    let next = [0x68; 16];
    // A group rekey under management frame protection carries the IGTK too.
    let kd = [&gtk_kde(2, &[0x22; 16])[..], &igtk_kde(&IGTK), &bigtk_kde(7, 1, &next)].concat();
    let f = crate::soft::frames::eapol_from(
        crate::ap_sim::AA,
        crate::ap_sim::SPA,
        &group1(Akm::Psk, 3, &ptk, &kd),
    );
    let _ = s.receive(&f, false);
    let refused = BeaconRx::Refused(nonos_wifi_core::station::MgmtDrop::Undecryptable);
    assert_eq!(s.receive_beacon(&signed(6, 51, &BIGTK)), refused, "the old key is gone");
    assert_eq!(s.receive_beacon(&signed(7, 2, &next)), BeaconRx::Verified, "from BIPN 1 up");
}
