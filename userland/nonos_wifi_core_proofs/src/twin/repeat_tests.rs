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

//! A key repeated is not handed to the driver again either: a driver that
//! installs the group key in its chip (`Rx::Handshake::group_key`) would
//! reset the chip's own counter. A group message 1 with the key in use, and
//! message 3 sent again, give none; a new group key gives it once; and an
//! AP rotating back to a key the station still holds gives none either.

use crate::ap_sim::{group1, gtk_kde, message3, ANONCE, AP_RSNE_MIXED, GTK, SPA};
use crate::receive::frame::from_ap;
use crate::receive::join::{eapol_body, joined};
use nonos_wifi_core::station::Rx;
use nonos_wifi_core::wpa::akm::Akm;

#[test]
fn a_repeated_key_is_never_handed_to_the_driver_again() {
    let (mut s, ptk) = joined();
    let tk: [u8; 16] = ptk[32..48].try_into().unwrap();
    let mut pn = 0;
    let mut take = |s: &mut nonos_wifi_core::station::LinkStation, eapol: Vec<u8>| {
        pn += 1;
        match s.receive(&from_ap(SPA, &eapol_body(&eapol), Some(&tk), pn, 0, true), false) {
            Rx::Handshake { frame, group_key, .. } => (frame.is_some(), group_key.map(|k| k.0)),
            _ => (false, None),
        }
    };
    let kd = [&AP_RSNE_MIXED[..], &gtk_kde(1, &GTK)].concat();
    assert_eq!(take(&mut s, message3(Akm::Psk, 3, &ANONCE, &ptk, &kd)), (true, None));
    assert_eq!(take(&mut s, group1(Akm::Psk, 4, &ptk, &gtk_kde(1, &GTK))), (true, None));
    assert_eq!(take(&mut s, group1(Akm::Psk, 5, &ptk, &gtk_kde(2, &[0x7e; 16]))), (true, Some(2)));
    assert_eq!(take(&mut s, group1(Akm::Psk, 6, &ptk, &gtk_kde(2, &[0x7e; 16]))), (true, None));
    // Back to the key the join installed, which slot 1 still holds: not a new
    // key, so not handed over again (802.11-2020, 12.7.7.3).
    assert_eq!(take(&mut s, group1(Akm::Psk, 7, &ptk, &gtk_kde(1, &GTK))), (true, None));
}
