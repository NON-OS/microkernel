// NONOS Operating System (AGPL-3.0-or-later)

//! The security that keys a network's address is the one its join runs:
//! read from the beacon as the MLME reads it, so a network offering PSK and
//! SAE is the SAE network, and one refused sends nothing under any address.

use nonos_wifi_core::address::Security;
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::wpa::akm::Akm;

use super::sim::{beacon, rsne};
use super::sim::{join, AP};

const MFPC: u16 = 0x80;
const MFPR: u16 = 0x40;

fn kind(elements: &[&[u8]], policy: JoinPolicy) -> Security {
    Security::of_beacon(&beacon(AP, elements), policy)
}

/// What the MLME itself runs on the same beacon, and whether it sent.
fn mlme(elements: &[&[u8]], policy: JoinPolicy) -> (Option<Akm>, bool) {
    let mut m = join(policy);
    let out = m.on_mgmt(&beacon(AP, elements));
    (m.akm(), out.tx.is_some())
}

#[test]
fn the_kind_is_the_akm_the_join_runs() {
    let cases: [(Vec<u8>, JoinPolicy, Security); 7] = [
        (rsne(&[2], 0, 4), JoinPolicy::ANY, Security::Psk),
        (rsne(&[6], MFPC, 4), JoinPolicy::ANY, Security::Psk),
        (rsne(&[8], MFPC | MFPR, 4), JoinPolicy::ANY, Security::Sae),
        (rsne(&[2, 8], MFPC, 4), JoinPolicy::ANY, Security::Sae),
        (rsne(&[2, 8], MFPC, 4), JoinPolicy::WPA3_ONLY, Security::Sae),
        (rsne(&[2, 8], 0, 4), JoinPolicy::ANY, Security::Psk),
        (rsne(&[2, 8], 0, 4), JoinPolicy::PSK_ONLY, Security::Psk),
    ];
    for (r, policy, want) in cases {
        assert_eq!(kind(&[&r], policy), want, "{r:?}");
        let (akm, sent) = mlme(&[&r], policy);
        assert!(sent, "{r:?} joins");
        assert_eq!(akm == Some(Akm::Sae), want == Security::Sae, "{r:?} agrees with the MLME");
    }
}

#[test]
fn a_network_the_join_refuses_sends_nothing_under_its_kind() {
    let refused: [(Vec<u8>, JoinPolicy, Security); 4] = [
        (Vec::new(), JoinPolicy::ANY, Security::Open),
        (vec![48, 2, 1, 0], JoinPolicy::ANY, Security::Psk),
        (rsne(&[2], 0, 4), JoinPolicy::WPA3_ONLY, Security::Psk),
        (rsne(&[8], 0, 4), JoinPolicy::ANY, Security::Psk),
    ];
    for (r, policy, want) in refused {
        let elements: &[&[u8]] = if r.is_empty() { &[] } else { &[&r] };
        assert_eq!(kind(elements, policy), want, "{r:?}");
        assert_eq!(mlme(elements, policy), (None, false), "{r:?} refused at the beacon");
    }
}
