// NONOS Operating System (AGPL-3.0-or-later)
//! The group key's index survives the four-way handshake. A simulated AP
//! drives the real `Supplicant` through messages 1 and 3 with the GTK KDE
//! naming index 1, then index 2, and the proof holds the supplicant (and the
//! MLME accessor the drivers read) to the index the AP sent. Group-addressed
//! frames name their key by this index and an AP moves it between 1 and 2 on
//! every rekey, so a driver that installs the group key anywhere else stops
//! decrypting broadcast (ARP, DHCP, router advertisements) after the first one.

use nonos_wifi_core::ccmp::aes::Aes128;
use nonos_wifi_core::eapol::build::build_key_frame;
use nonos_wifi_core::eapol::parse::{KEY_INFO_MIC, KEY_INFO_PAIRWISE, KEY_INFO_VERSION2};
use nonos_wifi_core::wpa::ptk::{pmk, ptk};
use nonos_wifi_core::wpa::supplicant::{State, Supplicant};

const AA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
const SPA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
const ANONCE: [u8; 32] = [0xA1; 32];
const SNONCE: [u8; 32] = [0x52; 32];
const GTK: [u8; 16] = [
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00,
];

// RFC 3394 AES key wrap: the AP's half of the group-key delivery.
fn aes_wrap(kek: &[u8; 16], plain: &[u8]) -> Vec<u8> {
    assert!(plain.len().is_multiple_of(8) && !plain.is_empty());
    let n = plain.len() / 8;
    let aes = Aes128::new(kek);
    let mut a = [0xA6u8; 8];
    let mut r: Vec<[u8; 8]> =
        (0..n).map(|i| plain[i * 8..i * 8 + 8].try_into().unwrap()).collect();
    for j in 0..6u64 {
        for (i, ri) in r.iter_mut().enumerate() {
            let mut block = [0u8; 16];
            block[..8].copy_from_slice(&a);
            block[8..].copy_from_slice(&ri[..]);
            aes.encrypt_block(&mut block);
            let t = (n as u64) * j + (i as u64) + 1;
            a.copy_from_slice(&block[..8]);
            for (k, ak) in a.iter_mut().enumerate() {
                *ak ^= (t >> (56 - 8 * k)) as u8;
            }
            ri.copy_from_slice(&block[8..]);
        }
    }
    let mut out = a.to_vec();
    for block in &r {
        out.extend_from_slice(block);
    }
    out
}

// GTK KDE: dd <len> 00 0f ac 01 <key id / tx> <reserved> <gtk>, wrapped.
fn wrapped_gtk_kde(kek: &[u8; 16], key_id_byte: u8) -> Vec<u8> {
    let mut kde = vec![0xDD, 22, 0x00, 0x0f, 0xac, 0x01, key_id_byte, 0x00];
    kde.extend_from_slice(&GTK);
    aes_wrap(kek, &kde)
}

fn message1() -> Vec<u8> {
    let mut out = [0u8; 160];
    let info = KEY_INFO_VERSION2 | KEY_INFO_PAIRWISE;
    let n = build_key_frame(&mut out, info, &[0, 0, 0, 0, 0, 0, 0, 1], &ANONCE, &[], &[0u8; 16])
        .unwrap();
    out[..n].to_vec()
}

fn message3(kck: &[u8], kek: &[u8; 16], key_id_byte: u8) -> Vec<u8> {
    let mut out = [0u8; 160];
    let info = KEY_INFO_VERSION2 | KEY_INFO_PAIRWISE | KEY_INFO_MIC;
    let kd = wrapped_gtk_kde(kek, key_id_byte);
    let n = build_key_frame(&mut out, info, &[0, 0, 0, 0, 0, 0, 0, 2], &ANONCE, &kd, kck).unwrap();
    out[..n].to_vec()
}

fn handshake(key_id_byte: u8) -> Supplicant {
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let p = ptk(&key, &AA, &SPA, &ANONCE, &SNONCE);
    let mut sup = Supplicant::new(key, AA, SPA, SNONCE);
    sup.step(&message1());
    assert_eq!(sup.state(), State::PtkDerived);
    sup.step(&message3(&p[0..16], p[16..32].try_into().unwrap(), key_id_byte));
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
