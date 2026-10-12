// NONOS Operating System (AGPL-3.0-or-later)
//! The group key's index survives the four-way handshake. A simulated AP
//! drives the real `Supplicant` through messages 1 and 3 with the GTK KDE
//! naming index 1, then index 2, and the proof holds the supplicant (and the
//! MLME accessor the drivers read) to the index the AP sent. Group-addressed
//! frames name their key by this index and an AP moves it between 1 and 2 on
//! every rekey, so a driver that installs the group key anywhere else stops
//! decrypting broadcast (ARP, DHCP, router advertisements) after the first one.

use crate::ap_sim::{gtk_kde, message1, message3, AA, ANONCE, GTK, SNONCE, SPA, STA_RSNE_PSK};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::{pmk, ptk};
use nonos_wifi_core::wpa::supplicant::{Config, State, Supplicant};

// A WPA2-PSK handshake whose message 3 signs the beacon's RSNE back with the
// GTK KDE whose key ID octet is `key_id_byte`.
fn handshake(key_id_byte: u8) -> Supplicant {
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let p = ptk(&key, &AA, &SPA, &ANONCE, &SNONCE);
    let mut sup = Supplicant::configure(&Config {
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
    });
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    assert_eq!(sup.state(), State::PtkDerived);
    let kd = [&STA_RSNE_PSK[..], &gtk_kde(key_id_byte, &GTK)].concat();
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &p, &kd));
    assert_eq!(sup.state(), State::Connected);
    assert_eq!(sup.gtk(), &GTK);
    sup
}

#[test]
fn the_group_key_keeps_the_index_the_ap_sent() {
    assert_eq!(handshake(0x01).gtk_id(), 1);
    assert_eq!(handshake(0x02).gtk_id(), 2, "the index after an AP's first rekey");
    assert_eq!(handshake(0x03).gtk_id(), 3);
}

#[test]
fn only_the_index_bits_are_taken() {
    // Bit 2 of the byte is the Tx flag, not part of the index.
    assert_eq!(handshake(0x04 | 0x02).gtk_id(), 2);
}
