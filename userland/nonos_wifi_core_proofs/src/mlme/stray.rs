// NONOS Operating System (AGPL-3.0-or-later)
//! A stray SAE Authentication frame during an Open System join is ignored:
//! one frame from anyone on the air does not end the join, which goes on to
//! Connected after it.

use nonos_wifi_core::eapol::mic::verify_mic_kind;
use nonos_wifi_core::mlme::MlmeState;
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::wpa::akm::Akm;

use super::frames::assoc_resp;
use super::frames::{open_auth_ok, sae_from_ap};
use super::sim::{beacon, rsne};
use super::sim::{join, AP, SNONCE, STA};
use crate::ap_sim::{gtk_kde, message1, message3, GTK};

#[test]
fn a_stray_sae_frame_leaves_an_open_system_join_running_to_connected() {
    let psk_rsne = rsne(&[2], 0, 4);
    let mut m = join(JoinPolicy::ANY);
    m.on_mgmt(&beacon(AP, &[&psk_rsne])).tx.expect("open authentication");
    for (seq, status) in [(1, 0), (1, 1), (2, 0), (2, 15), (1, 126)] {
        let out = m.on_mgmt(&sae_from_ap(seq, status, &[0xAB; 96]));
        assert!(out.tx.is_none(), "SAE seq {seq} status {status}: nothing sent");
        assert_eq!(m.state(), MlmeState::Authenticating, "seq {seq} status {status}");
    }
    m.on_mgmt(&open_auth_ok(AP, STA)).tx.expect("association request");
    assert!(m.on_mgmt(&assoc_resp(0)).tx.is_none());
    let key = nonos_wifi_core::wpa::ptk::pmk(super::sim::PASS, super::sim::SSID);
    let anonce = [0xA1; 32];
    let m2 = m.on_eapol(&message1(Akm::Psk, 1, &anonce)).tx.expect("message 2");
    let ptk = Akm::Psk.derive_ptk(&key, &AP, &STA, &anonce, &SNONCE);
    assert!(verify_mic_kind(Akm::Psk.mic_kind(), &ptk[..16], &m2));
    let mut kd = psk_rsne.clone();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    assert!(m.on_eapol(&message3(Akm::Psk, 2, &anonce, &ptk, &kd)).tx.is_some());
    assert_eq!(m.state(), MlmeState::Connected);
}
