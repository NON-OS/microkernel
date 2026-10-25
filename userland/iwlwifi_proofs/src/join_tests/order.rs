// NONOS Operating System (AGPL-3.0-or-later)
//! A WPA2 and a WPA3 join run Linux's command order to an open port.

use nonos_wifi_core::wpa::akm::Akm;

use super::drive::{joined, only_handshake_frames_left};
use super::rig::JOIN_COMMANDS;
use crate::ap_sim::GTK;
use crate::join_rig::*;

#[test]
fn a_wpa2_join_runs_linuxs_command_order_to_an_open_port() {
    let r = rig(Security::Wpa2, Switches::default());
    let (akm, p) = joined(&r);
    assert_eq!(akm, Akm::Psk);
    assert_eq!(r.commands(), JOIN_COMMANDS.to_vec());
    // Authentication, association, messages 2 and 4.
    let kinds: Vec<u8> = r.sent().iter().map(|s| s.frame[0]).collect();
    assert_eq!(kinds, vec![0xB0, 0x00, 0x08, 0x08]);
    only_handshake_frames_left(&r);
    assert_eq!((p.sent, p.eapol), (4, 2));

    // The keys in the firmware are the ones the access point derived.
    let ap = r.ap.borrow();
    assert!(ap.keyed, "the access point took message 4");
    let keys = r.command_payloads(0x5, 0x18);
    assert_eq!(&keys[0][16..32], &ap.tk(), "pairwise key, index 0");
    assert_eq!((keys[0][8], keys[0][12]), (0, 0x02), "no management protection on WPA2");
    assert_eq!(&keys[1][16..32], &GTK, "group key");
    assert_eq!((keys[1][8], keys[1][12]), (1, 0x22), "at the index the access point named");
    // Associated with AID 1, the station entry authorized without MFP.
    let macs = r.command_payloads(0x3, 0x08);
    assert_eq!((macs[0][36], macs[0][40]), (1, 1));
    let stas = r.command_payloads(0x3, 0x0A);
    assert_eq!((stas[0][36], stas[1][28], stas[1][36], stas[2][36]), (1, 1, 1, 0));
    // The link was activated on the PHY with the BSS's ACK rates, then took
    // the association's ERP preamble.
    let links = r.command_payloads(0x3, 0x09);
    assert_eq!((links[1][24], links[1][28], links[1][36], links[1][40]), (3, 1, 0x0F, 0x15));
    assert_eq!((links[2][24], links[2][44]), (2, 1));
    assert_eq!(&links[1][136..142], &[100, 0, 0, 0, 200, 0], "beacon interval and DTIM interval");
}

#[test]
fn a_wpa3_join_runs_sae_both_ways_to_an_open_port_with_protected_management() {
    for h2e in [true, false] {
        let r = rig(Security::Sae { h2e }, Switches::default());
        let (akm, _) = joined(&r);
        assert_eq!(akm, Akm::Sae, "h2e={h2e}");
        assert_eq!(r.commands(), JOIN_COMMANDS.to_vec());
        let kinds: Vec<u8> = r.sent().iter().map(|s| s.frame[0]).collect();
        assert_eq!(kinds, vec![0xB0, 0xB0, 0x00, 0x08, 0x08], "commit, confirm, association, 2, 4");
        only_handshake_frames_left(&r);
        let keys = r.command_payloads(0x5, 0x18);
        assert_eq!(&keys[0][16..32], &r.ap.borrow().tk());
        assert_eq!(keys[0][12], 0x42, "pairwise key with management protection");
        let stas = r.command_payloads(0x3, 0x0A);
        assert_eq!(stas[2][36], 1, "authorized with MFP");
    }
}
