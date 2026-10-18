// NONOS Operating System (AGPL-3.0-or-later)

//! An SAE join fed the access point's commit and confirm, mutated. Nothing
//! may panic, and no frame altered past its unauthenticated header fields
//! may carry the join past authentication: the confirm binds both commits,
//! so only the AP's own pair can.

use nonos_wifi_core::dot11::header::MAC_HEADER_LEN;
use nonos_wifi_core::mlme::MlmeState;
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::sae::hnp::derive_pwe;
use nonos_wifi_core::sae::{SaeStation, SaeStep};

use super::frames::sae_from_ap;
use super::sim::beacon;
use super::sim::{join, AP, PASS, STA};
use crate::ap_sim::AP_RSNE_MIXED;
use crate::sweep::fuzz::{altered, Rng};

const ROUNDS: u32 = 1_000;

// Whether `m` changed what the frame says, not only the header fields no
// station authenticates: the frame-control flags, duration, sequence control.
fn altered_mgmt(seed: &[u8], m: &[u8]) -> bool {
    let mask = |f: &[u8]| -> Vec<u8> {
        let skip = |i: usize| matches!(i, 1..=3 | 22 | 23);
        f.iter().enumerate().map(|(i, &b)| if skip(i) { 0 } else { b }).collect()
    };
    altered(&mask(seed), &mask(m))
}

#[test]
fn altered_sae_frames_never_authenticate() {
    let mut m = join(JoinPolicy::ANY);
    let commit = m.on_mgmt(&beacon(AP, &[&AP_RSNE_MIXED])).tx.expect("an SAE commit");
    let pwe = derive_pwe(PASS, &vec![9u8; PASS.len()], &AP, &STA).unwrap();
    let mut ap = SaeStation::start(pwe, false, &[0x33; 32], &[0x44; 32]).unwrap();
    let SaeStep::Send { body: ap_confirm, .. } = ap.on_frame(1, 0, &commit[MAC_HEADER_LEN + 6..])
    else {
        panic!("the AP accepts the station's commit");
    };
    let (_, status, ap_commit) = ap.commit_frame();
    let seeds = [sae_from_ap(1, status, &ap_commit), sae_from_ap(2, 0, &ap_confirm)];
    let mut r = Rng::seeded(0x5AE);
    for round in 0..ROUNDS {
        // A station waiting for the AP's commit, or (odd rounds) for its
        // confirm after the AP's own commit.
        let mut m = join(JoinPolicy::ANY);
        m.on_mgmt(&beacon(AP, &[&AP_RSNE_MIXED]));
        let seed = &seeds[round as usize % 2];
        if round % 2 == 1 {
            assert!(m.on_mgmt(&seeds[0]).tx.is_some(), "the station confirms");
        }
        let f = r.mutate(seed);
        m.on_mgmt(&f);
        if altered_mgmt(seed, &f) {
            let state = m.state();
            assert!(
                matches!(state, MlmeState::Authenticating | MlmeState::Failed),
                "round {round}"
            );
        }
    }
}
