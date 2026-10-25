// NONOS Operating System (AGPL-3.0-or-later)
//! A connected supplicant with a rekey pending, fed mutated EAPOL-Key frames:
//! a repeated message 3, a group message 1, the rekey's message 3 and a
//! message 1, twenty thousand rounds. Nothing may panic, and no frame a MIC
//! covers, once altered, may end the link or change a key, nor be answered
//! while it still claims a MIC. Bytes appended
//! past a frame's own length are padding, not part of it.

use nonos_wifi_core::eapol::mic::compute_mic;
use nonos_wifi_core::eapol::parse::{parse, KEY_INFO_MIC};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::{pmk, ptk};
use nonos_wifi_core::wpa::supplicant::State;

use super::forged_m1_tests::supplicant;
use crate::ap_sim::{
    group1, gtk_kde, message1, message3, AA, ANONCE, GTK, SNONCE, SPA, STA_RSNE_PSK,
};
use crate::sweep::fuzz::{altered, Rng};

const ROUNDS: u32 = 20_000;
const ANONCE2: [u8; 32] = [0x5A; 32];

// `template` under replay counter `n`, its MIC under `kck` made again: a
// frame the supplicant has not seen, so every round reaches the MIC check.
fn fresh(template: &[u8], kck: &[u8], n: u64) -> Vec<u8> {
    let mut f = template.to_vec();
    f[9..17].copy_from_slice(&n.to_be_bytes());
    f[81..97].fill(0);
    let mic = compute_mic(kck, &f);
    f[81..97].copy_from_slice(&mic);
    f
}

#[test]
fn altered_frames_never_end_the_link_or_change_a_key() {
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let ap = ptk(&key, &AA, &SPA, &ANONCE, &SNONCE);
    let mut sup = supplicant(key);
    let kd = [&STA_RSNE_PSK[..], &gtk_kde(1, &GTK)].concat();
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ap, &kd));
    assert_eq!(sup.state(), State::Connected);
    let m2 = sup.step(&message1(Akm::Psk, 3, &ANONCE2)).reply.expect("the rekey's message 2");
    let snonce2 = parse(&m2).expect("message 2").nonce;
    let next = ptk(&key, &AA, &SPA, &ANONCE2, &snonce2);
    let seeds = [
        (message3(Akm::Psk, 9, &ANONCE, &ap, &kd), &ap[..16]),
        (group1(Akm::Psk, 9, &ap, &gtk_kde(2, &[0x22; 16])), &ap[..16]),
        (message3(Akm::Psk, 9, &ANONCE2, &next, &kd), &next[..16]),
    ];
    let (mut tk, mut gtk) = (sup.tk().to_vec(), sup.gtk().to_vec());
    let mut r = Rng::seeded(0x5E9);
    for round in 0..ROUNDS {
        let (template, kck) = &seeds[round as usize % 3];
        let seed = &fresh(template, kck, 10 + u64::from(round));
        let f = r.mutate(seed);
        let out = sup.step(&f);
        if altered(seed, &f) {
            // One that still claims a MIC is dropped unanswered; one that no
            // longer does reads as a message 1, answered and nothing more.
            let claims_mic = parse(&f).is_some_and(|k| k.key_info & KEY_INFO_MIC != 0);
            assert!(!(claims_mic && out.reply.is_some()), "round {round}: answered");
            assert!(!out.new_group_key && !out.new_pairwise_key, "round {round}");
            assert_eq!((sup.tk(), sup.gtk()), (&tk[..], &gtk[..]), "round {round}");
            assert_eq!(sup.state(), State::Connected, "round {round}");
        } else {
            // Only padding past the frame's own length: the AP's frame.
            (tk, gtk) = (sup.tk().to_vec(), sup.gtk().to_vec());
        }
        if round % 64 == 0 {
            sup.step(&r.mutate(&message1(Akm::Psk, 9, &ANONCE2)));
        }
    }
}
