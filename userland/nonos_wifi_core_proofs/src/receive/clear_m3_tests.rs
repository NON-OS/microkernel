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

//! A message 3 repeated in the clear, before the AP has the pairwise key, is
//! answered in the clear with message 4 again, and no key is reinstalled.

use crate::ap_sim::{gtk_kde, message3, ANONCE, AP_RSNE_MIXED, GTK};
use crate::receive::frame::{from_ap, STA};
use crate::receive::join::{eapol_body, joined};
use nonos_wifi_core::station::Rx;
use nonos_wifi_core::wpa::akm::Akm;

#[test]
fn a_repeated_message3_in_the_clear_is_answered_in_the_clear() {
    let (mut s, ptk) = joined();
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    let m3 = message3(Akm::Psk, 3, &ANONCE, &ptk, &kd);
    let f = from_ap(STA, &eapol_body(&m3), None, 0, 0, false);
    let Rx::Handshake { frame, group_key, pairwise_key } = s.receive(&f, false) else {
        panic!("handled")
    };
    assert!(group_key.is_none() && pairwise_key.is_none(), "no key is reinstalled");
    let reply = frame.expect("message 4 again");
    assert_eq!(reply[1] & 0x40, 0, "in the clear: the AP has no pairwise key yet");
}
