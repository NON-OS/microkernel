// NONOS Operating System (AGPL-3.0-or-later)
//! A message 1 carries no MIC, so anyone can send one. One forged between the
//! AP's message 1 and its message 3 no longer ends the handshake: message 3 is
//! checked under the PTK of the ANonce it repeats, and the keys installed are
//! the AP's.

use crate::ap_sim::{gtk_kde, message1, message3, AA, ANONCE, GTK, SNONCE, SPA, STA_RSNE_PSK};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::{pmk, ptk};
use nonos_wifi_core::wpa::supplicant::{Config, State, Supplicant};

pub(crate) fn supplicant(key: [u8; 32]) -> Supplicant {
    Supplicant::configure(&Config {
        pmk: key,
        aa: AA,
        spa: SPA,
        snonce: SNONCE,
        akm: Akm::Psk,
        own_rsne: &STA_RSNE_PSK,
        own_rsnxe: None,
        ap_rsne: &STA_RSNE_PSK,
        ap_rsnxe: None,
        pmf: false,
    })
}

#[test]
fn a_forged_message_1_before_message_3_does_not_end_the_handshake() {
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let ap = ptk(&key, &AA, &SPA, &ANONCE, &SNONCE);
    let mut sup = supplicant(key);
    assert!(sup.step(&message1(Akm::Psk, 1, &ANONCE)).reply.is_some());
    assert!(sup.step(&message1(Akm::Psk, 9, &[0xF0; 32])).reply.is_some(), "it is answered");
    let kd = [&STA_RSNE_PSK[..], &gtk_kde(1, &GTK)].concat();
    assert!(sup.step(&message3(Akm::Psk, 2, &ANONCE, &ap, &kd)).reply.is_some(), "message 4");
    assert_eq!(sup.state(), State::Connected);
    assert_eq!(sup.tk(), &ap[32..48], "the AP's key, not the forger's");
}

#[test]
fn a_message_3_keyed_by_the_forgers_anonce_alone_is_still_refused() {
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let wrong = ptk(&pmk(b"not the password", b"ThisIsASSID"), &AA, &SPA, &[0xF0; 32], &SNONCE);
    let mut sup = supplicant(key);
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let kd = [&STA_RSNE_PSK[..], &gtk_kde(1, &GTK)].concat();
    assert!(sup.step(&message3(Akm::Psk, 2, &[0xF0; 32], &wrong, &kd)).reply.is_none());
    assert_eq!(sup.state(), State::PtkDerived, "still waiting for the AP's");
}
