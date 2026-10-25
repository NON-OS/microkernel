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

//! Row 1.6: nothing of the session goes out while the station is off its
//! channel, and from the Reassociation Request until the new keys are in
//! only the move's own frames go, in the clear: management frames and the
//! four-way handshake. A candidate that accepts and then never keys is
//! given up on at the join's bound.

use super::msg3_tests::last_roam;
use crate::roam::two_aps::{run_until, sent, two_aps, BB};
use crate::soft::frames::eapol_of;
use crate::soft::wire::netif;
use nonos_wifi_core::netif::wire::OP_TX_PACKET;
use nonos_wifi_core::softmac::control::roam_code::CODE_AP_UNPROVEN;
use nonos_wifi_core::softmac::roam::COOLDOWN_MS;

const ETH: [u8; 20] = [2, 0, 0, 0, 0, 9, 2, 0, 0, 0, 0, 2, 0x08, 0, 0x45, 0, 0, 20, 0, 0];

#[test]
fn net_cores_frames_wait_while_the_station_is_away() {
    let mut st = two_aps(|_| {}, [-85, -50]);
    let away = |s: &crate::soft::rig::Station| s.radio().unwrap().tuned.map(|t| t.1) != Some(1);
    assert!(run_until(&mut st, away), "a look took the station away");
    let before = st.radio().unwrap().sent.len();
    netif(&mut st, OP_TX_PACKET, &ETH);
    assert_eq!(st.radio().unwrap().sent.len(), before, "nothing goes out away");
}

#[test]
fn from_the_reassociation_only_the_moves_frames_go_in_the_clear() {
    let mut st = two_aps(|_| {}, [-85, -50]);
    assert!(run_until(&mut st, |s| s.session().is_some_and(|s| s.bssid == BB)));
    let sent = &st.radio().unwrap().sent;
    let at = sent.iter().position(|f| f[0] == 0x20).unwrap();
    for f in &sent[at..] {
        let mgmt = f[0] & 0x0c == 0;
        let eapol = f[0] & 0x0c == 0x08 && f[1] & 0x40 == 0 && eapol_of(f).is_some();
        assert!(mgmt || eapol, "only the move's frames: {:02x?}", &f[..2]);
    }
}

#[test]
fn a_candidate_that_accepts_and_never_keys_is_given_up_on() {
    let mut st = two_aps(|b| b.quirks.mute_eapol = true, [-85, -40]);
    let clock = |s: &crate::soft::rig::Station| s.radio().unwrap().clock.get();
    let mut before = clock(&st);
    while sent(&st, 0x20).is_empty() && clock(&st) < 2 * COOLDOWN_MS {
        before = clock(&st);
        st.tick();
    }
    assert!(st.session().is_none(), "the move ended without a session");
    assert!(clock(&st) - before < COOLDOWN_MS, "given up on long before the next look");
    assert_eq!(last_roam(&mut st), CODE_AP_UNPROVEN);
}
