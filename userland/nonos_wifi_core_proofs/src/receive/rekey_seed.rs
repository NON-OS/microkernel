// NONOS Operating System (AGPL-3.0-or-later)

//! The rekey SNonce's seed: a message 1 sent again for the same ANonce is
//! answered with the same SNonce, as nothing is kept between messages; a new
//! seed gives a new SNonce for that same ANonce, and the rekey still takes.

use nonos_wifi_core::station::LinkStation;
use nonos_wifi_core::wpa::akm::Akm;

use super::join::joined;
use super::rekey::{feed, tk, ANONCE2};
use crate::ap_sim::{gtk_kde, message1, message3, AA, AP_RSNE_MIXED, GTK, SPA};
use nonos_wifi_core::wpa::ptk::pmk;

// The SNonce of the message 2 a message 1 for ANONCE2 draws.
fn snonce(s: &mut LinkStation, key: &[u8; 16], replay: u8, pn: u64) -> [u8; 32] {
    let (m2, _) = feed(s, &message1(Akm::Psk, replay, &ANONCE2), key, pn);
    m2[17..49].try_into().unwrap()
}

#[test]
fn a_new_seed_gives_a_new_snonce_and_the_rekey_still_takes() {
    let (mut s, ptk) = joined();
    let old = tk(&ptk);
    let first = snonce(&mut s, &old, 3, 2);
    assert_eq!(snonce(&mut s, &old, 4, 3), first, "message 1 again: the same SNonce");
    s.reseed_rekey([0x9E; 32]);
    let second = snonce(&mut s, &old, 5, 4);
    assert_ne!(second, first, "a new seed, a new SNonce for the same ANonce");
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let new_ptk = Akm::Psk.derive_ptk(&key, &AA, &SPA, &ANONCE2, &second);
    let kd = [&AP_RSNE_MIXED[..], &gtk_kde(1, &GTK)].concat();
    let (_, pairwise) = feed(&mut s, &message3(Akm::Psk, 6, &ANONCE2, &new_ptk, &kd), &old, 5);
    assert_eq!(pairwise, Some(tk(&new_ptk)), "under the SNonce last sent");
}
