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

//! The receive half of an associated link.
//!
//! CCMP is checked against IEEE Std 802.11-2012 M.6.4 (the CCMP test vector:
//! input as in hostapd wlantest/test_vectors.c test_vector_ccmp(), fetched from
//! https://w1.fi/cgit/hostap/plain/wlantest/test_vectors.c, sha256
//! 9aceafbe03d32ca537dab030de0350c5d9cc658de176bc21c76ab95b7e30a633; the
//! expected encrypted MPDU is the published one). The QoS vector was computed
//! independently with the Python `cryptography` AES-CCM and hostapd's
//! ccmp_aad_nonce() rules (TID in the nonce, QoS Control with only the TID in
//! the AAD), since the Annex vector is non-QoS and access points send QoS data.
//!
//! Then `LinkStation::receive`: frames from the BSS are delivered, everything
//! else (another BSS, ToDS, another station, fragments, A-MSDUs, unprotected
//! data on a protected link, replays, the station's own broadcasts coming
//! back) is dropped with its reason, a chip-decrypted frame loses its CCMP
//! header and MIC, and the AP's group key handshake is answered and its key
//! taken.

use crate::ap_sim::{gtk_kde, group1, message1, message3, AA, ANONCE, AP_RSNE_MIXED, GTK, SNONCE, SPA};
use nonos_wifi_core::ccmp::ccm::ccm_encrypt;
use nonos_wifi_core::dot11::ccmp::{aad_nonce, decrypt, view};
use nonos_wifi_core::dot11::data::protect;
use nonos_wifi_core::station::{Ccmp, LinkStation, Rx, RxDrop};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::supplicant::{Config, Supplicant};

fn h(s: &str) -> Vec<u8> {
    let s: String = s.split_whitespace().collect();
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

const TK_M64: &str = "c97c1f67ce371185514a8a19f2bdd52f";
const M64_PLAIN: &str = "0848c32c0fd2e128a57c5030f1844408abaea5b8fcba8033
    f8ba1a55d02f85ae967bb62fb6cda8eb7e78a050";
const M64_CIPHER: &str = "0848c32c0fd2e128a57c5030f1844408abaea5b8fcba8033
    0ce7002076970 3b5 f3d0a2fe9a3dbf2342a643e43246e80c3c04d0197845ce0b16f97623";
const QOS_CIPHER: &str = "88420000020000000002020000000001020000000077301225000201002000000000
    be61194719d3a6a5a42a2593b793f93ec4bb74166420a43f7a";

fn tk() -> [u8; 16] {
    h(TK_M64).try_into().unwrap()
}

#[test]
fn ccmp_matches_the_annex_m64_vector() {
    let enc = protect(&h(M64_PLAIN), 0xB5039776E70C, &tk()).unwrap();
    assert_eq!(enc, h(M64_CIPHER), "encrypted MPDU");
    assert_eq!(decrypt(&enc, &tk()).unwrap(), h(M64_PLAIN)[24..].to_vec(), "decrypts back");
}

#[test]
fn a_qos_frame_decrypts_with_its_tid_in_nonce_and_aad() {
    let f = h(QOS_CIPHER);
    let v = view(&f).unwrap();
    assert_eq!((v.hdr_len, v.tid), (26, 5));
    let plain = decrypt(&f, &tk()).expect("the QoS frame decrypts");
    assert_eq!(&plain[..8], &h("aaaa030000000800")[..]);
    assert_eq!(&plain[8..], b"hello qos");
    let mut flipped = f.clone();
    flipped[24] ^= 0x01; // another TID: a different nonce and AAD
    assert!(decrypt(&flipped, &tk()).is_none(), "the TID is authenticated");
}

const STA: [u8; 6] = SPA;
const AP: [u8; 6] = AA;
const SRC: [u8; 6] = [0x02, 0, 0, 0, 0, 0x77];

// An AP-to-station frame: FromDS, QoS (TID 0) when `qos`, `body` from the
// LLC/SNAP header on, encrypted under `key` at `pn` with key index `kid`
// unless `key` is None.
fn from_ap(dst: [u8; 6], body: &[u8], key: Option<&[u8; 16]>, pn: u64, kid: u8, qos: bool) -> Vec<u8> {
    let mut f = vec![if qos { 0x88 } else { 0x08 }, 0x02, 0, 0];
    f.extend_from_slice(&dst);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&SRC);
    f.extend_from_slice(&[0x10, 0x00]);
    if qos {
        f.extend_from_slice(&[0x00, 0x00]);
    }
    let Some(key) = key else {
        f.extend_from_slice(body);
        return f;
    };
    f[1] |= 0x40;
    let hdr_len = f.len();
    let p = pn.to_be_bytes();
    f.extend_from_slice(&[p[7], p[6], 0, 0x20 | (kid << 6), p[5], p[4], p[3], p[2]]);
    let v = view(&f).unwrap();
    let mut aad = [0u8; 32];
    let (nonce, n) = aad_nonce(&f, &v, pn, &mut aad);
    let mut ct = vec![0u8; body.len() + 8];
    ccm_encrypt(key, &nonce, &aad[..n], body, &mut ct).unwrap();
    assert_eq!(hdr_len, v.hdr_len);
    f.extend_from_slice(&ct);
    f
}

fn ip_body(payload: &[u8]) -> Vec<u8> {
    let mut b = h("aaaa030000000800");
    b.extend_from_slice(payload);
    b
}

fn eapol_body(eapol: &[u8]) -> Vec<u8> {
    let mut b = h("aaaa0300000088 8e");
    b.extend_from_slice(eapol);
    b
}

fn station() -> LinkStation {
    let mut s = LinkStation::new(STA);
    s.associate(AP, Ccmp::Software { tk: tk() });
    s
}

#[test]
fn a_protected_frame_from_the_bss_becomes_ethernet() {
    let mut s = station();
    for qos in [false, true] {
        let f = from_ap(STA, &ip_body(b"payload"), Some(&tk()), if qos { 2 } else { 1 }, 0, qos);
        let Rx::Ethernet(eth) = s.receive(&f, false) else { panic!("delivered, qos={qos}") };
        assert_eq!(&eth[0..6], &STA, "destination");
        assert_eq!(&eth[6..12], &SRC, "source is the FromDS Address 3");
        assert_eq!(&eth[12..14], &[0x08, 0x00]);
        assert_eq!(&eth[14..], b"payload");
    }
}

#[test]
fn a_chip_decrypted_frame_loses_its_ccmp_header_and_mic() {
    let mut s = station();
    // The chip hands up the frame decrypted in place: CCMP header, plaintext, MIC.
    let mut f = from_ap(STA, &[], None, 0, 0, true);
    f[1] |= 0x40;
    f.extend_from_slice(&[5, 0, 0, 0x20, 0, 0, 0, 0]);
    f.extend_from_slice(&ip_body(b"chip"));
    f.extend_from_slice(&[0xEE; 8]);
    let Rx::Ethernet(eth) = s.receive(&f, true) else { panic!("delivered") };
    assert_eq!(&eth[14..], b"chip", "no MIC bytes trail the payload");
    assert!(matches!(s.receive(&f, true), Rx::Dropped(RxDrop::Replay)), "replay checked too");
}

#[test]
fn frames_that_are_not_for_this_link_are_dropped_with_a_reason() {
    let mut s = station();
    let good = from_ap(STA, &ip_body(b"x"), Some(&tk()), 1, 0, true);
    let mut other_bss = good.clone();
    other_bss[10..16].copy_from_slice(&[0x02, 0, 0, 0, 0, 0x55]);
    assert!(matches!(s.receive(&other_bss, false), Rx::Dropped(RxDrop::NotFromBss)));
    let mut to_ds = good.clone();
    to_ds[1] = (to_ds[1] & !0x03) | 0x01;
    assert!(matches!(s.receive(&to_ds, false), Rx::Dropped(RxDrop::NotFromBss)));
    let mut other_sta = good.clone();
    other_sta[4..10].copy_from_slice(&[0x02, 0, 0, 0, 0, 0x33]);
    assert!(matches!(s.receive(&other_sta, false), Rx::Dropped(RxDrop::NotForUs)));
    let mut fragment = good.clone();
    fragment[1] |= 0x04;
    assert!(matches!(s.receive(&fragment, false), Rx::Dropped(RxDrop::Fragmented)));
    let mut amsdu = good.clone();
    amsdu[24] |= 0x80;
    assert!(matches!(s.receive(&amsdu, false), Rx::Dropped(RxDrop::Amsdu)));
    let plain = from_ap(STA, &ip_body(b"injected"), None, 0, 0, true);
    assert!(matches!(s.receive(&plain, false), Rx::Dropped(RxDrop::Unprotected)));
    let mut tampered = good.clone();
    let last = tampered.len() - 1;
    tampered[last] ^= 1;
    assert!(matches!(s.receive(&tampered, false), Rx::Dropped(RxDrop::Undecryptable)));
    assert!(matches!(s.receive(&good[..30], false), Rx::Dropped(RxDrop::Malformed)));
    assert!(matches!(s.receive(&good, false), Rx::Ethernet(_)));
    assert!(matches!(s.receive(&good, false), Rx::Dropped(RxDrop::Replay)));
    let mut unassociated = LinkStation::new(STA);
    assert!(matches!(unassociated.receive(&good, false), Rx::Dropped(RxDrop::NotAssociated)));
}

// A station that finished a WPA2 join: its supplicant, the PTK the AP holds.
fn joined() -> (LinkStation, [u8; 48]) {
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let mut sup = Supplicant::configure(&Config {
        pmk: key,
        aa: AA,
        spa: SPA,
        snonce: SNONCE,
        akm: Akm::Psk,
        own_rsne: &nonos_wifi_core::wpa::RSN_IE,
        own_rsnxe: None,
        ap_rsne: &AP_RSNE_MIXED,
        ap_rsnxe: None,
        pmf: false,
    });
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let ptk = Akm::Psk.derive_ptk(&key, &AA, &SPA, &ANONCE, &SNONCE);
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    let mut m3 = message3(Akm::Psk, 2, &ANONCE, &ptk, &kd);
    m3[65] = 9; // Key RSC: the AP's group frames are past PN 9
    let m3 = {
        // Recompute the MIC over the edited RSC.
        let mic = nonos_wifi_core::eapol::mic::compute_mic(&ptk[..16], &m3);
        let mut f = m3;
        f[81..97].copy_from_slice(&mic);
        f
    };
    sup.step(&m3);
    let mut tkb = [0u8; 16];
    tkb.copy_from_slice(&ptk[32..48]);
    let mut s = LinkStation::new(STA);
    s.associate(AP, Ccmp::Software { tk: tkb });
    s.set_supplicant(sup);
    (s, ptk)
}

#[test]
fn group_frames_decrypt_under_the_gtk_above_its_rsc() {
    let (mut s, _) = joined();
    let bcast = [0xFF; 6];
    let old = from_ap(bcast, &ip_body(b"arp"), Some(&GTK), 9, 1, false);
    assert!(matches!(s.receive(&old, false), Rx::Dropped(RxDrop::Replay)), "at the RSC");
    let new = from_ap(bcast, &ip_body(b"arp"), Some(&GTK), 10, 1, false);
    assert!(matches!(s.receive(&new, false), Rx::Ethernet(_)));
    let wrong_index = from_ap(bcast, &ip_body(b"arp"), Some(&GTK), 11, 2, false);
    assert!(matches!(s.receive(&wrong_index, false), Rx::Dropped(RxDrop::Undecryptable)));
    let mut echo = from_ap(bcast, &ip_body(b"arp"), Some(&GTK), 12, 1, false);
    echo[16..22].copy_from_slice(&STA);
    assert!(matches!(s.receive(&echo, false), Rx::Dropped(RxDrop::NotForUs)), "our own");
}

#[test]
fn the_group_key_handshake_is_answered_on_the_link() {
    let (mut s, ptk) = joined();
    let next = [0x22u8; 16];
    let mut tkb = [0u8; 16];
    tkb.copy_from_slice(&ptk[32..48]);
    let g1 = group1(Akm::Psk, 3, &ptk, &gtk_kde(2, &next));
    let f = from_ap(STA, &eapol_body(&g1), Some(&tkb), 1, 0, true);
    let Rx::Handshake { frame, group_key } = s.receive(&f, false) else { panic!("handled") };
    assert_eq!(group_key, Some((2, next)), "the new key for the driver to install");
    let reply = frame.expect("group message 2");
    assert_ne!(reply[1] & 0x40, 0, "sent protected, as the message came");
    let plain = decrypt(&reply, &tkb).expect("under the pairwise key");
    assert_eq!(&plain[..8], &h("aaaa03000000888e")[..]);
    // Frames under the new key now decrypt.
    let bcast = from_ap([0xFF; 6], &ip_body(b"new"), Some(&next), 1, 2, false);
    assert!(matches!(s.receive(&bcast, false), Rx::Ethernet(_)));
}

#[test]
fn a_repeated_message3_in_the_clear_is_answered_in_the_clear() {
    let (mut s, ptk) = joined();
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    let m3 = message3(Akm::Psk, 3, &ANONCE, &ptk, &kd);
    let f = from_ap(STA, &eapol_body(&m3), None, 0, 0, false);
    let Rx::Handshake { frame, group_key } = s.receive(&f, false) else { panic!("handled") };
    assert!(group_key.is_none(), "no key is reinstalled");
    let reply = frame.expect("message 4 again");
    assert_eq!(reply[1] & 0x40, 0, "in the clear: the AP has no pairwise key yet");
}
