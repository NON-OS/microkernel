// NONOS Operating System (AGPL-3.0-or-later)

//! The rig of the key reinstallation proofs: a link joined to the AP, its
//! four-way handshake finished, with the AP's later EAPOL waiting on the ring.

use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::supplicant::Supplicant;

use super::card::Card;
use super::group::supplicant;
use super::receive::{chip_decrypted, W0_ENC_AES};
use super::rig::{eth_frame, link_with, Dma, AP_MAC, STA_MAC};
use super::rxbuf::RxBuf;
use crate::ap_sim::{gtk_kde, message1, message3, ANONCE, AP_RSNE_MIXED, GTK, SNONCE};
use crate::link::RtlLink;

// The four-way handshake finished: the supplicant, its PTK and its TK.
pub(crate) fn joined() -> (Supplicant, [u8; 48], [u8; 16]) {
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let mut sup = supplicant(false);
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let ptk = Akm::Psk.derive_ptk(&key, &AP_MAC, &STA_MAC, &ANONCE, &SNONCE);
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &msg3_kd()));
    let mut tk = [0u8; 16];
    tk.copy_from_slice(&ptk[32..48]);
    let mut p = [0u8; 48];
    p.copy_from_slice(&ptk[..48]);
    (sup, p, tk)
}

pub(crate) fn msg3_kd() -> Vec<u8> {
    [AP_RSNE_MIXED.to_vec(), gtk_kde(1, &GTK)].concat()
}

pub(crate) type Link = RtlLink<Card, Dma, RxBuf>;

// A link joined to the AP, with the given EAPOL messages waiting on the ring.
pub(crate) fn link_after_join(eapols: &[Vec<u8>]) -> (Link, Card) {
    let (sup, _, tk) = joined();
    let mut rx = RxBuf::new(0x4000_0000);
    for (i, m) in eapols.iter().enumerate() {
        let eth = eth_frame(STA_MAC, AP_MAC, [0x88, 0x8E], m);
        rx.stage_w0(i as u32, &chip_decrypted(&eth, i as u8 + 1), W0_ENC_AES);
    }
    let card = Card::new(0, eapols.len() as u32);
    let probe = card.clone();
    let (mut link, _tx) = link_with(card, rx);
    link.associate(AP_MAC, tk);
    link.set_supplicant(sup);
    (link, probe)
}
