// NONOS Operating System (AGPL-3.0-or-later)

//! A forged rekey message 3 (its MIC under no key the AP holds) changes
//! nothing: the old key stays in use and the rekey can still take; nor does
//! a forged message 1 between the AP's message 1 and its message 3.

use nonos_wifi_core::station::Rx;
use nonos_wifi_core::wpa::akm::Akm;

use super::frame::{from_ap, STA};
use super::join::eapol_body;
use super::join::joined;
use super::rekey::{data_ok, feed, rekey1, tk, ANONCE2};
use crate::ap_sim::{gtk_kde, message1, message3, AP_RSNE_MIXED, GTK};

#[test]
fn a_forged_rekey_changes_nothing() {
    let (mut s, ptk) = joined();
    let old = tk(&ptk);
    let new_ptk = rekey1(&mut s, &old);
    let kd = [&AP_RSNE_MIXED[..], &gtk_kde(1, &GTK)].concat();
    let forged = message3(Akm::Psk, 4, &ANONCE2, &[0xEE; 48], &kd);
    let f = from_ap(STA, &eapol_body(&forged), Some(&old), 3, 0, true);
    assert!(matches!(s.receive(&f, false), Rx::Handshake { frame: None, pairwise_key: None, .. }));
    assert!(data_ok(&mut s, &old, 4), "still the old key");
    let (_, pairwise) = feed(&mut s, &message3(Akm::Psk, 4, &ANONCE2, &new_ptk, &kd), &old, 5);
    assert_eq!(pairwise, Some(tk(&new_ptk)), "the pending rekey survived the forgery");
}

#[test]
fn a_forged_message_1_during_a_rekey_does_not_stop_it() {
    let (mut s, ptk) = joined();
    let old = tk(&ptk);
    let new_ptk = rekey1(&mut s, &old);
    let (m2, pairwise) = feed(&mut s, &message1(Akm::Psk, 4, &[0xF0; 32]), &old, 3);
    assert!(pairwise.is_none() && !m2.is_empty(), "answered, as message 1 carries no MIC");
    let kd = [&AP_RSNE_MIXED[..], &gtk_kde(1, &GTK)].concat();
    let (_, pairwise) = feed(&mut s, &message3(Akm::Psk, 5, &ANONCE2, &new_ptk, &kd), &old, 4);
    assert_eq!(pairwise, Some(tk(&new_ptk)), "the AP's rekey still takes");
}
