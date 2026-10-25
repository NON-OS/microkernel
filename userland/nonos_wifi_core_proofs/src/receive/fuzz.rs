// NONOS Operating System (AGPL-3.0-or-later)

//! A joined link fed mutated frames from the access point: data, the group
//! key handshake, a repeated message 3 and a rekey's message 1. Nothing may
//! panic; a frame delivered must decrypt under the pairwise key by the
//! reference CCMP; no pairwise key is ever handed out, a group key only as
//! the AP sent it; and afterwards the link still takes the AP's frames.

use nonos_wifi_core::dot11::ccmp::decrypt;
use nonos_wifi_core::station::Rx;
use nonos_wifi_core::wpa::akm::Akm;

use super::frame::{from_ap, STA};
use super::join::joined;
use super::join::{eapol_body, ip_body};
use super::rekey::{data_ok, tk, ANONCE2};
use crate::ap_sim::{group1, gtk_kde, message1, message3, ANONCE, AP_RSNE_MIXED, GTK};
use crate::sweep::fuzz::Rng;

const ROUNDS: u32 = 5_000;

#[test]
fn a_link_fed_mutated_frames_keeps_its_keys() {
    let (mut s, ptk) = joined();
    let key = tk(&ptk);
    let next = [0x22u8; 16];
    let kd = [&AP_RSNE_MIXED[..], &gtk_kde(1, &GTK)].concat();
    let seeds = [
        from_ap(STA, &ip_body(b"payload"), Some(&key), 20, 0, true),
        from_ap([0xFF; 6], &ip_body(b"arp"), Some(&GTK), 20, 1, false),
        from_ap(
            STA,
            &eapol_body(&group1(Akm::Psk, 3, &ptk, &gtk_kde(2, &next))),
            Some(&key),
            21,
            0,
            true,
        ),
        from_ap(
            STA,
            &eapol_body(&message3(Akm::Psk, 4, &ANONCE, &ptk, &kd)),
            Some(&key),
            22,
            0,
            true,
        ),
        from_ap(STA, &eapol_body(&message1(Akm::Psk, 5, &ANONCE2)), Some(&key), 23, 0, true),
    ];
    let mut r = Rng::seeded(0x11F0);
    for round in 0..ROUNDS {
        let f = r.mutate(&seeds[round as usize % seeds.len()]);
        match s.receive(&f, false) {
            Rx::Ethernet(_) => {
                let unicast = decrypt(&f, &key).is_some();
                assert!(
                    unicast || decrypt(&f, &GTK).is_some() || decrypt(&f, &next).is_some(),
                    "round {round}"
                );
            }
            Rx::Handshake { pairwise_key, group_key, .. } => {
                assert!(pairwise_key.is_none(), "round {round}");
                assert!(group_key.is_none_or(|g| g == (2, next)), "round {round}");
            }
            _ => {}
        }
    }
    assert!(data_ok(&mut s, &key, 1 << 40), "the pairwise key is still the one in use");
}
