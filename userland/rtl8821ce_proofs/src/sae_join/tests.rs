// NONOS Operating System (AGPL-3.0-or-later)

//! A whole WPA3 join through this driver's own join loop (`run_join`): SAE by
//! hash to element against an access point built from the shared SAE code,
//! association with management frame protection, then the four-way handshake
//! to `Joined`, holding the pairwise key the SAE PMK keys and the AP's group
//! key. A wrong password, against the same AP, never joins.

use super::ap::SaeAp;
use super::frames::{beacon, PASS, SSID, STA};
use crate::assoc::{run_join, Outcome};
use nonos_wifi_core::mlme::{Entropy, JoinRequest, SAE_ENTROPY};
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::wpa::akm::Akm;

fn request(passphrase: &'static [u8]) -> JoinRequest<'static> {
    let mut sae = [0u8; SAE_ENTROPY];
    sae.iter_mut().enumerate().for_each(|(i, b)| *b = (i as u8).wrapping_mul(37).wrapping_add(11));
    let entropy = Entropy { snonce: [0x52; 32], sae, rekey: [0x6B; 32] };
    JoinRequest { our_mac: STA, ssid: SSID, passphrase, policy: JoinPolicy::ANY, entropy }
}

// A clock that moves one millisecond each time it is read.
fn clock() -> impl FnMut() -> u64 {
    let mut t = 0;
    move || {
        t += 1;
        t
    }
}

#[test]
fn a_wpa3_join_runs_sae_and_the_four_way_through_the_driver_to_joined() {
    let mut ap = SaeAp::new();
    let report = run_join(&mut ap, &request(PASS), &beacon(), 10_000, &mut clock());
    let Outcome::Joined { ptk, gtk, gtk_id, akm, supplicant, .. } = report.outcome else {
        panic!("not joined: {:?}", report.failure);
    };
    assert_eq!(akm, Akm::Sae, "WPA3, not the WPA2 the AP also offers");
    assert_eq!(&ptk.0[..], &ap.ptk[32..48], "the TK of the PTK the SAE PMK derives");
    assert_eq!((gtk.0, gtk_id), ([0x11; 16], 1), "the AP's group key");
    assert!(supplicant.is_some(), "kept for the AP's later group key handshakes");
    let auths = ap.sent_by_sta.iter().filter(|f| f[0] == 0xB0).count();
    assert_eq!(auths, 2, "an SAE commit and an SAE confirm, nothing else");
}

#[test]
fn a_wrong_password_never_joins_and_sends_no_handshake() {
    let mut ap = SaeAp::new();
    let report = run_join(&mut ap, &request(b"wrong horse"), &beacon(), 10_000, &mut clock());
    assert!(!matches!(report.outcome, Outcome::Joined { .. }), "joined with the wrong password");
    let data = ap.sent_by_sta.iter().filter(|f| f[0] & 0x0C == 0x08).count();
    assert_eq!(data, 0, "no association, so no handshake frame went out");
    assert!(ap.ptk.is_empty());
}
