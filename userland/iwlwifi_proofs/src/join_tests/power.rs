// NONOS Operating System (AGPL-3.0-or-later)
//! A join sends the station's power command twice, as Linux's
//! `iwl_mvm_power_update_mac` is called: on association with power
//! management off, and once the keys are in and the session is cancelled
//! with it on, so the handshake runs with the radio awake.

use super::drive::joined;
use crate::join_rig::*;

#[test]
fn a_join_turns_power_management_on_only_once_the_keys_are_in() {
    let r = rig(Security::Wpa2, Switches::default());
    joined(&r);
    let sent = r.command_payloads(0x1, 0xA9);
    assert_eq!(sent.len(), 2, "on association, then once authorized");
    assert_eq!(&sent[0][4..6], &[1, 0], "associated: power save allowed, not managed");
    assert_eq!(&sent[1][4..6], &[3, 0], "authorized: power management on");
    let keep_alive = |c: &[u8]| u16::from_le_bytes([c[6], c[7]]);
    assert!(sent.iter().all(|c| keep_alive(c) >= 25), "the keep-alive floor");
}
