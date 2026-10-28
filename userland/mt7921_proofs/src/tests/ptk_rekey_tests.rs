// NONOS Operating System (AGPL-3.0-or-later)

//! A pairwise rekey on the model, the access point's messages sealed under
//! the key in use: message 4 goes out sealed by the chip under that key,
//! then the new key goes in and data under it comes up. A new key the
//! firmware refuses ends the link, since the chip still holds the old one.

use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;

use crate::ap_sim::{gtk_kde, message1, message3, GTK};
use crate::model::ap_tx::{hear, ipv4, sealed, unicast};
use crate::model::net::{joined, PASS, SSID};
use crate::model::rig::Station;
use crate::model::wire::{netif, NET_CORE};
use crate::model::Shared;
use crate::soft::data_frames::eapol_from;
use crate::soft::frames::RSNE_PSK;

const ANONCE2: [u8; 32] = [0xb2; 32];

/// The rekey to message 3; the new pairwise key.
fn rekey(st: &mut Station, c: &Shared) -> [u8; 16] {
    let (bssid, sta) = {
        let chip = c.borrow();
        let air = chip.air.as_ref().unwrap();
        (air.ap.bssid, air.sta)
    };
    let llc = |m: &[u8]| eapol_from(bssid, sta, m)[24..].to_vec();
    hear(st, c, unicast(c, &llc(&message1(Akm::Psk, 3, &ANONCE2)), 20));
    let m2 = c.borrow().air.as_ref().unwrap().ap.got.last().unwrap().clone();
    let snonce: [u8; 32] = m2[8 + 17..8 + 49].try_into().unwrap();
    let ptk = Akm::Psk.derive_ptk(&pmk(PASS, SSID), &bssid, &sta, &ANONCE2, &snonce);
    let kd = [&RSNE_PSK[..], &gtk_kde(1, &GTK)].concat();
    hear(st, c, unicast(c, &llc(&message3(Akm::Psk, 4, &ANONCE2, &ptk, &kd)), 21));
    ptk[32..48].try_into().unwrap()
}

#[test]
fn a_pairwise_rekey_puts_the_new_key_in_the_chip_after_message_4() {
    let (mut st, c) = joined();
    let tk = rekey(&mut st, &c);
    let (bssid, sta, m4) = {
        let chip = c.borrow();
        let air = chip.air.as_ref().unwrap();
        assert_eq!(chip.keys.entries[&1].pairwise, Some(tk), "the new key, in the chip");
        (air.ap.bssid, air.sta, air.ap.got.last().unwrap().clone())
    };
    assert!(m4.starts_with(&[0xaa, 0xaa, 3, 0, 0, 0, 0x88, 0x8e]), "message 4, under the old key");
    let up = hear(&mut st, &c, sealed(bssid, sta, &ipv4(b"under the new key"), 1, &tk, 0));
    assert!(up.iter().any(|e| e.ends_with(b"under the new key")), "{up:?}");
}

#[test]
fn a_new_pairwise_key_the_firmware_refuses_ends_the_link() {
    let (mut st, c) = joined();
    c.borrow_mut().faults.refuse_keys = true;
    rekey(&mut st, &c);
    assert_eq!(netif(&mut st, NET_CORE, 2, &[]).1, [0], "the link is down");
}
