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
//! A chip reset that puts the radio's own address back: a reconnect takes
//! the network's again before its first authentication, a chip that then
//! refuses it joins nothing, and a move to another AP is not made from it.

use super::join_tests::from_auth;
use crate::roam::two_aps::{run_until, sent, two_aps};
use crate::soft::air::STA;
use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::soft::join::{sta_of, PASS, SSID};
use crate::soft::rig::rig;
use crate::soft::wire::{code, connect, netif};
use nonos_wifi_core::netif::wire::OP_MAC_ADDRESS;
use nonos_wifi_core::softmac::control::result::CODE_NO_ADDRESS;

#[test]
fn a_reconnect_after_a_reset_takes_the_networks_address_again() {
    let mut st = rig(SimAp::new(SSID, PASS, 6, &RSNE_PSK));
    assert_eq!(code(&connect(&mut st, SSID, PASS, 0)), 0);
    let net = sta_of(&st);
    st.radio_mut().unwrap().addr.0 = STA;
    st.radio_mut().unwrap().sent.clear();
    assert_eq!(code(&connect(&mut st, SSID, PASS, 0)), 0);
    assert!(from_auth(&st).iter().all(|a| *a == net), "the network's address again");
    st.radio_mut().unwrap().addr = (STA, true);
    st.radio_mut().unwrap().sent.clear();
    assert_eq!(code(&connect(&mut st, SSID, PASS, 0)), CODE_NO_ADDRESS);
    let sent = &st.radio().unwrap().sent;
    assert!(sent.iter().all(|f| f[0] != 0xb0), "no authentication from the chip's own");
}

#[test]
fn a_move_is_not_made_from_a_radio_that_lost_the_networks_address() {
    let mut st = two_aps(|_| {}, [-85, -50]);
    let air = st.radio_mut().unwrap();
    (air.chat_to, air.addr.0) = (Some(air.addr.0), STA);
    let joined = air.sent.len();
    assert!(run_until(&mut st, |s| s.session().is_none()), "the session ends");
    assert!(sent(&st, 0x20).is_empty(), "no reassociation");
    let since = &st.radio().unwrap().sent[joined..];
    assert!(!since.is_empty() && since.iter().all(|f| f[10..16] != STA), "none from its own");
}

#[test]
fn net_core_is_told_the_sessions_address_whatever_the_chip_holds() {
    let mut st = rig(SimAp::new(SSID, PASS, 6, &RSNE_PSK));
    assert_eq!(code(&connect(&mut st, SSID, PASS, 0)), 0);
    let net = sta_of(&st);
    st.radio_mut().unwrap().addr.0 = STA;
    assert_eq!(netif(&mut st, OP_MAC_ADDRESS, &[]), (0, net.to_vec()));
}
