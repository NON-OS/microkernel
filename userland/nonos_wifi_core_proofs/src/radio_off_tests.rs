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

//! E-72: a radio turned off sends nothing. The RADIO op off freezes every
//! transmit path — the join (and its hidden probe), the data plane, and the
//! between-request upkeep (session pump, roam, scan) — so no frame leaves;
//! on resumes, from a fresh local address.

use crate::roam::two_aps::two_aps;
use crate::soft::join::{joined, PASS, SSID};
use crate::soft::rig::Station;
use crate::soft::wire::{code, connect, control, netif};
use nonos_wifi_core::netif::wire::OP_TX_PACKET;
use nonos_wifi_core::softmac::engine::OP_RADIO;

fn eth() -> Vec<u8> {
    [&[2, 0, 0, 0, 0, 9][..], &[2, 0, 0, 0, 0, 2], &[8, 0], &[7u8; 24]].concat()
}

fn sent(st: &Station) -> usize {
    st.radio().unwrap().sent.len()
}

#[test]
fn off_stops_the_join_and_data_paths_then_on_resumes() {
    let mut st = joined();
    assert_eq!(code(&control(&mut st, OP_RADIO, &[0])), 0, "turned off");
    st.radio_mut().unwrap().sent.clear();
    assert!(code(&connect(&mut st, SSID, PASS, 0)) < 0, "connect refused while off");
    assert!(netif(&mut st, OP_TX_PACKET, &eth()).0 < 0, "data refused while off");
    for _ in 0..80 {
        st.tick();
    }
    assert_eq!(sent(&st), 0, "a transmit path escaped while off");
    assert_eq!(code(&control(&mut st, OP_RADIO, &[1])), 0, "turned on");
    assert_eq!(code(&connect(&mut st, SSID, PASS, 0)), 0, "joins again once on");
    assert!(sent(&st) > 0, "nothing sent after on");
    assert_eq!(st.radio().unwrap().addr.0[0] & 0x03, 0x02, "a local unicast address");
}

#[test]
fn off_suppresses_the_roam_that_ticks_would_otherwise_send() {
    // BB far stronger than AA: with the radio on, ticks look and reassociate.
    let mut st = two_aps(|_| {}, [-85, -50]);
    assert_eq!(code(&control(&mut st, OP_RADIO, &[0])), 0, "turned off");
    st.radio_mut().unwrap().sent.clear();
    for _ in 0..3000 {
        st.tick();
    }
    assert_eq!(sent(&st), 0, "a roam escaped while off");
    assert_eq!(code(&control(&mut st, OP_RADIO, &[1])), 0, "turned on");
    for _ in 0..3000 {
        st.tick();
    }
    assert!(sent(&st) > 0, "no look after turning on");
}
