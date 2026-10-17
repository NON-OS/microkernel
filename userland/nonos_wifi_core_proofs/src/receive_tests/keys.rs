// NONOS Operating System (AGPL-3.0-or-later)

//! Group frames and the key handshakes on the link.

use nonos_wifi_core::dot11::ccmp::decrypt;
use nonos_wifi_core::station::{Rx, RxDrop};
use nonos_wifi_core::wpa::akm::Akm;

use super::frames::{eapol_body, from_ap, ip_body, STA};
use super::stations::joined;
use super::vectors::h;
use crate::ap_sim::{group1, gtk_kde, message3, ANONCE, AP_RSNE_MIXED, GTK};

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
