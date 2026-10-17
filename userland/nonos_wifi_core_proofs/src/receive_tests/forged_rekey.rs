// NONOS Operating System (AGPL-3.0-or-later)

//! A forged rekey message 3 (its MIC under no key the AP holds) changes
//! nothing: the old key stays in use and the rekey stays pending.

use nonos_wifi_core::station::Rx;
use nonos_wifi_core::wpa::akm::Akm;

use super::frames::{eapol_body, from_ap, STA};
use super::rekey::{data_ok, feed, rekey1, tk, ANONCE2};
use super::stations::joined;
use crate::ap_sim::{gtk_kde, message3, AP_RSNE_MIXED, GTK};

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
