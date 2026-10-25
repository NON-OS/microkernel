// NONOS Operating System (AGPL-3.0-or-later)

//! The supplicant the MLME hands the data path is seeded for its rekeys from
//! the join's randomness: two joins alike but for that seed answer the same
//! rekey's message 1 with different SNonces.

use nonos_wifi_core::eapol::parse::parse;
use nonos_wifi_core::mlme::{JoinRequest, Mlme, MlmeState};
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;

use super::frames::assoc_resp;
use super::frames::open_auth_ok;
use super::sim::{beacon, rsne};
use super::sim::{entropy, AP, PASS, SNONCE, SSID, STA};
use crate::ap_sim::{gtk_kde, message1, message3, GTK};

// A WPA2 join run to Connected with `seed` as its rekey seed; the SNonce its
// supplicant answers a rekey's message 1 with.
fn rekey_snonce(seed: [u8; 32]) -> [u8; 32] {
    let mut e = entropy();
    e.rekey = seed;
    let policy = JoinPolicy::ANY;
    let mut m =
        Mlme::join(&JoinRequest { our_mac: STA, ssid: SSID, passphrase: PASS, policy, entropy: e });
    let psk = rsne(&[2], 0, 4);
    m.on_mgmt(&beacon(AP, &[&psk]));
    m.on_mgmt(&open_auth_ok(AP, STA));
    m.on_mgmt(&assoc_resp(0));
    let anonce = [0xA1; 32];
    m.on_eapol(&message1(Akm::Psk, 1, &anonce));
    let ptk = Akm::Psk.derive_ptk(&pmk(PASS, SSID), &AP, &STA, &anonce, &SNONCE);
    let kd = [&psk[..], &gtk_kde(1, &GTK)].concat();
    m.on_eapol(&message3(Akm::Psk, 2, &anonce, &ptk, &kd));
    assert_eq!(m.state(), MlmeState::Connected);
    let mut sup = m.supplicant().expect("connected");
    let m2 = sup.step(&message1(Akm::Psk, 3, &[0x5A; 32])).reply.expect("message 2");
    parse(&m2).expect("an EAPOL-Key frame").nonce
}

#[test]
fn the_rekey_seed_comes_from_the_joins_randomness() {
    assert_eq!(rekey_snonce([1; 32]), rekey_snonce([1; 32]), "one seed, one SNonce");
    assert_ne!(rekey_snonce([1; 32]), rekey_snonce([2; 32]), "a seed of its own");
}
