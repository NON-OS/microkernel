// NONOS Operating System (AGPL-3.0-or-later)
//! Association end to end: the shared station scans, authenticates,
//! associates and runs the four-way handshake with the simulated AP, every
//! frame through the driver's rings and the modelled chip's air.

use super::station::{code, connect, on_air, PASS, SSID};

#[test]
fn joins_wpa2_on_channel_6() {
    let (card, mut st) = on_air(6);
    let r = connect(&mut st, SSID, PASS);
    let s = card.0.borrow();
    assert_eq!(s.violations, Vec::<String>::new());
    assert_eq!((code(&r), r[33]), (0, 2), "joined with WPA2-PSK");
    let air = s.air.as_ref().unwrap();
    assert!(air.ap.tk.is_some(), "the AP installed the pairwise key on message 4");
    // Authentication and association went out on the management queue,
    // the handshake's EAPOL frames on the data queue.
    let mgmt: Vec<u8> = s.sent.iter().filter(|f| f.ch == 8).map(|f| f.frame[0]).collect();
    assert!(mgmt.contains(&0xb0) && mgmt.contains(&0x00), "auth and assoc on CH8: {mgmt:x?}");
    assert!(s.sent.iter().filter(|f| f.ch == 0).count() >= 2, "messages 2 and 4 on ACH0");
    // The chip is on channel 6, 2.4 GHz, as RF path A was programmed.
    assert_eq!(crate::model::rf_tuned(&s), Some((false, 6)));
}

#[test]
fn joins_on_5ghz_channel_36() {
    let (card, mut st) = on_air(36);
    let r = connect(&mut st, SSID, PASS);
    let s = card.0.borrow();
    assert_eq!(s.violations, Vec::<String>::new());
    assert_eq!((code(&r), r[33]), (0, 2));
    assert_eq!(crate::model::rf_tuned(&s), Some((true, 36)));
}

#[test]
fn a_disconnect_tells_the_ap_and_the_firmware_then_a_rejoin_works() {
    let (card, mut st) = on_air(6);
    assert_eq!(code(&connect(&mut st, SSID, PASS)), 0);
    assert_eq!(code(&super::station::control(&mut st, 2, &[])), 0);
    {
        let s = card.0.borrow();
        let last = s.sent.last().unwrap();
        assert_eq!((last.ch, last.frame[0]), (8, 0xc0), "deauthentication on CH8");
        let join = s.h2c.iter().rev().find(|c| c.is(1, 8, 0)).unwrap();
        let w = u32::from_le_bytes(join.body[..4].try_into().unwrap());
        assert_eq!((w & 0x100, (w >> 24) & 3), (0x100, 0), "disconnected, no link");
        assert_eq!(s.cam.unwrap().1, [0; 6], "the BSSID out of the CAM");
    }
    assert_eq!(code(&connect(&mut st, SSID, PASS)), 0, "and back again");
    assert_eq!(card.0.borrow().violations, Vec::<String>::new());
}

#[test]
fn wrong_passphrase_is_refused() {
    let (card, mut st) = on_air(6);
    let r = connect(&mut st, SSID, b"NotThePassword1");
    assert_ne!(code(&r), 0);
    assert!(card.0.borrow().air.as_ref().unwrap().ap.tk.is_none());
}
