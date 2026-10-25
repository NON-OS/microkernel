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

//! The supplicant keeps the indication only from a message 3 that verified,
//! unwrapped under the PTK and passed the downgrade check.

use super::sim::{after_message1, connected, kd3, td_kde};
use crate::ap_sim::*;
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::supplicant::State;

#[test]
fn message_3_carries_the_indication() {
    let (sup, _) = connected(&kd3(&td_kde(0x01)));
    assert_eq!(sup.state(), State::Connected);
    assert_eq!(sup.transition_disable(), Some(0x01));
    let (none, _) = connected(&kd3(&[]));
    assert!(none.transition_disable().is_none(), "absent, nothing is invented");
}

#[test]
fn message_1_in_the_clear_is_not_taken() {
    let (mut sup, ptk) = after_message1(&td_kde(0x01));
    assert!(sup.transition_disable().is_none(), "not even before message 3");
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd3(&[])));
    assert_eq!(sup.state(), State::Connected);
    assert!(sup.transition_disable().is_none());
}

#[test]
fn a_group_key_message_neither_sets_nor_changes_it() {
    let (mut sup, ptk) = connected(&kd3(&[]));
    let kd = [&gtk_kde(2, &[0x22; 16])[..], &td_kde(0x01)].concat();
    assert!(sup.step(&group1(Akm::Psk, 3, &ptk, &kd)).reply.is_some(), "the rekey runs");
    assert!(sup.transition_disable().is_none());
    let (mut sup, ptk) = connected(&kd3(&td_kde(0x01)));
    let kd = [&gtk_kde(2, &[0x22; 16])[..], &td_kde(0x0f)].concat();
    sup.step(&group1(Akm::Psk, 3, &ptk, &kd));
    assert_eq!(sup.transition_disable(), Some(0x01));
}

#[test]
fn a_retransmitted_message_3_changes_nothing() {
    let (mut sup, ptk) = connected(&kd3(&td_kde(0x01)));
    assert!(sup.step(&message3(Akm::Psk, 3, &ANONCE, &ptk, &kd3(&td_kde(0x0f)))).reply.is_some());
    assert_eq!(sup.transition_disable(), Some(0x01));
}

#[test]
fn a_message_3_that_fails_is_not_taken() {
    let (sup, _) = connected(&[&STA_RSNE_PSK[..], &gtk_kde(1, &GTK), &td_kde(0x01)].concat());
    assert_eq!(sup.state(), State::Failed, "the RSNE differs from the beacon's");
    assert!(sup.transition_disable().is_none());
    let (mut sup, _) = after_message1(&[]);
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &[0x5a; 48], &kd3(&td_kde(0x01))));
    assert_ne!(sup.state(), State::Connected, "not under this handshake's PTK");
    assert!(sup.transition_disable().is_none());
}
