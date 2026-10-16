// NONOS Operating System (AGPL-3.0-or-later)

//! The group key handshake answered on the link.

use nonos_wifi_core::netif::LinkPort;
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;

use super::card::Card;
use super::receive::{chip_decrypted, W0_ENC_AES};
use super::rig::{eth_frame, link_with, AP_MAC, STA_MAC};
use super::rxbuf::RxBuf;
use crate::ap_sim::{group1, gtk_kde, message1, message3, ANONCE, AP_RSNE_MIXED, GTK, SNONCE};
use crate::tx::desc::TXDESC_LEN;
use crate::tx::regs::REG_TXBD_IDX_BEQ;
use nonos_wifi_core::wpa::supplicant::{Config, Supplicant};

// A supplicant configured for this link, with management frame protection
// as given.
pub(super) fn supplicant(pmf: bool) -> Supplicant {
    Supplicant::configure(&Config {
        pmk: pmk(b"ThisIsAPassword", b"ThisIsASSID"),
        aa: AP_MAC,
        spa: STA_MAC,
        snonce: SNONCE,
        akm: Akm::Psk,
        own_rsne: &nonos_wifi_core::wpa::RSN_IE,
        own_rsnxe: None,
        ap_rsne: &AP_RSNE_MIXED,
        ap_rsnxe: None,
        pmf,
    })
}

#[test]
fn the_group_key_handshake_is_answered_on_the_link() {
    // Finish a four-way handshake, so the supplicant holds the PTK and the
    // first group key.
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let mut sup = supplicant(false);
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let ptk = Akm::Psk.derive_ptk(&key, &AP_MAC, &STA_MAC, &ANONCE, &SNONCE);
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd));
    let mut tk = [0u8; 16];
    tk.copy_from_slice(&ptk[32..48]);

    // The AP rekeys: group message 1 arrives protected, decrypted by the chip.
    let next = [0x22u8; 16];
    let g1 = group1(Akm::Psk, 3, &ptk, &gtk_kde(2, &next));
    let eth = eth_frame(STA_MAC, AP_MAC, [0x88, 0x8E], &g1);
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage_w0(0, &chip_decrypted(&eth, 1), W0_ENC_AES);
    let card = Card::new(0, 1);
    let probe = card.clone();
    let (mut link, txm) = link_with(card, rx);
    link.associate(AP_MAC, tk);
    link.set_supplicant(sup);

    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none(), "nothing for the stack");
    assert_eq!(link.take_group_key(), Some((2, next)), "the new key, for the CAM");
    assert_eq!(link.take_group_key(), None, "handed over once");
    assert_eq!(link.stats().rekeys, 1);
    assert!(probe.wrote(REG_TXBD_IDX_BEQ), "group message 2 was queued");
    let mem = txm.mem.borrow();
    assert_ne!(mem[TXDESC_LEN + 1] & 0x40, 0, "sent protected, as the message came");
}
