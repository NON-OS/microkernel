// NONOS Operating System (AGPL-3.0-or-later)
//! EAPOL-Key frames, and authentication, deauthentication and association
//! responses.

use nonos_wifi_core::dot11::auth::{parse_auth, parse_leave};
use nonos_wifi_core::dot11::parse::parse_assoc_response;
use nonos_wifi_core::eapol::parse::parse;
use nonos_wifi_core::wpa::akm::Akm;

use super::parse_beacon_fuzz::ROUNDS;
use super::rng::within;
use crate::ap_sim::{gtk_kde, message1, message3, AA, ANONCE, AP_RSNE_MIXED, GTK, SPA};
use crate::sweep::fuzz::Rng;

#[test]
fn eapol_and_auth_parsers_stay_inside_the_frame() {
    let ptk = Akm::Psk.derive_ptk(&[7; 32], &AA, &SPA, &ANONCE, &[8; 32]);
    let m3 =
        message3(Akm::Psk, 2, &ANONCE, &ptk, &[&AP_RSNE_MIXED[..], &gtk_kde(1, &GTK)].concat());
    let seeds = [
        message1(Akm::Psk, 1, &ANONCE),
        m3,
        vec![0xB0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 1],
    ];
    let mut r = Rng::seeded(0xEA9013);
    for round in 0..ROUNDS {
        let f = r.mutate(&seeds[round as usize % 3]);
        if let Some(k) = parse(&f) {
            assert!(within(&f, k.key_data), "round {round}");
        }
        if let Some(a) = parse_auth(&f, &SPA, &AA) {
            assert!(within(&f, a.body), "round {round}");
        }
        let _ = (parse_leave(&f, &SPA, &AA), parse_assoc_response(&f));
    }
}
