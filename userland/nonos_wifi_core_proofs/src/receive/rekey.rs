// NONOS Operating System (AGPL-3.0-or-later)

//! A pairwise rekey on the link: the keys in use carry on until an authentic
//! message 3 for the new ANonce, its message 4 goes out under the old key, and
//! only then does the new key take over, packet numbers started afresh.

use nonos_wifi_core::dot11::ccmp::decrypt;
use nonos_wifi_core::eapol::mic::verify_mic;
use nonos_wifi_core::station::{LinkStation, Rx, RxDrop};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;

use super::frame::{from_ap, AP, STA};
use super::join::joined;
use super::join::{eapol_body, ip_body};
use crate::ap_sim::{gtk_kde, message1, message3, AA, AP_RSNE_MIXED, GTK, SPA};

pub(super) const ANONCE2: [u8; 32] = [0x5A; 32];

pub(super) fn tk(ptk: &[u8; 48]) -> [u8; 16] {
    ptk[32..48].try_into().unwrap()
}

// Feed `eapol` under `key`; the frame back, decrypted under `key`, and the
// pairwise key handed to the driver.
pub(super) fn feed(
    s: &mut LinkStation,
    eapol: &[u8],
    key: &[u8; 16],
    pn: u64,
) -> (Vec<u8>, Option<[u8; 16]>) {
    let f = from_ap(STA, &eapol_body(eapol), Some(key), pn, 0, true);
    let Rx::Handshake { frame, pairwise_key, .. } = s.receive(&f, false) else { panic!("handled") };
    let plain = decrypt(&frame.expect("answered"), key).expect("under the key in use");
    (plain[8..].to_vec(), pairwise_key)
}

// Message 1 for ANONCE2: message 2 back, and the PTK the AP derives from it.
pub(super) fn rekey1(s: &mut LinkStation, old: &[u8; 16]) -> [u8; 48] {
    let (m2, new) = feed(s, &message1(Akm::Psk, 3, &ANONCE2), old, 2);
    assert!(new.is_none(), "no key yet");
    let snonce: [u8; 32] = m2[17..49].try_into().unwrap();
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let ptk = Akm::Psk.derive_ptk(&key, &AA, &SPA, &ANONCE2, &snonce);
    assert!(verify_mic(&ptk[..16], &m2), "message 2 is keyed by the new PTK");
    ptk
}

pub(super) fn data_ok(s: &mut LinkStation, key: &[u8; 16], pn: u64) -> bool {
    matches!(
        s.receive(&from_ap(STA, &ip_body(b"x"), Some(key), pn, 0, true), false),
        Rx::Ethernet(_)
    )
}

#[test]
fn a_rekey_takes_the_new_key_only_after_its_message_3() {
    let (mut s, ptk) = joined();
    let old = tk(&ptk);
    let new_ptk = rekey1(&mut s, &old);
    assert!(data_ok(&mut s, &old, 3), "the old key carries on meanwhile");
    let kd = [&AP_RSNE_MIXED[..], &gtk_kde(1, &GTK)].concat();
    let (m4, pairwise) = feed(&mut s, &message3(Akm::Psk, 4, &ANONCE2, &new_ptk, &kd), &old, 4);
    assert_eq!(pairwise, Some(tk(&new_ptk)), "the new key for the driver");
    assert!(verify_mic(&new_ptk[..16], &m4), "message 4, sent under the old key");
    assert!(data_ok(&mut s, &tk(&new_ptk), 1), "the new key, its counter afresh");
    let stale = from_ap(STA, &ip_body(b"x"), Some(&old), 5, 0, true);
    assert!(matches!(s.receive(&stale, false), Rx::Dropped(RxDrop::Undecryptable)));
    let eth = [&AP[..], &STA, &[0x08, 0x00], b"out"].concat();
    let out = s.tx_frame(&eth).expect("framed");
    assert!(decrypt(&out, &tk(&new_ptk)).is_some(), "sent under the new key");
    assert_eq!(out[24], 1, "packet number 1");
}
