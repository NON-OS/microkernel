// NONOS Operating System (AGPL-3.0-or-later)

//! Which AKM a join takes, and what is refused.

use nonos_wifi_core::rsn::suite::CIPHER_TKIP;
use nonos_wifi_core::rsn::{parse_rsne, select, JoinPolicy, Pmf, SelectError};
use nonos_wifi_core::wpa::akm::Akm;

use super::parse::body;

#[test]
fn sae_is_chosen_whenever_offered() {
    let mixed = parse_rsne(&body(4, &[4], &[2, 8], 0x0080)).unwrap();
    let s = select(&mixed, true, JoinPolicy::ANY).unwrap();
    assert_eq!((s.akm, s.pmf, s.sae_h2e), (Akm::Sae, Pmf::Capable, true));
    let only = parse_rsne(&body(4, &[4], &[8], 0x00C0)).unwrap();
    let s = select(&only, false, JoinPolicy::ANY).unwrap();
    assert_eq!((s.akm, s.pmf, s.sae_h2e), (Akm::Sae, Pmf::Required, false));
    // A policy that allows only PSK takes it on a mixed network.
    let psk = JoinPolicy { allow_psk: true, allow_sae: false };
    assert_eq!(select(&mixed, true, psk).unwrap().akm, Akm::Psk);
}

#[test]
fn a_network_saved_as_wpa3_is_never_joined_with_psk() {
    let psk_only = parse_rsne(&body(4, &[4], &[2], 0)).unwrap();
    assert_eq!(select(&psk_only, false, JoinPolicy::WPA3_ONLY), Err(SelectError::Downgrade));
    assert_eq!(select(&psk_only, false, JoinPolicy::ANY).unwrap().akm, Akm::Psk);
    let sha256 = parse_rsne(&body(4, &[4], &[6], 0x0080)).unwrap();
    assert_eq!(select(&sha256, false, JoinPolicy::WPA3_ONLY), Err(SelectError::Downgrade));
    // SAE without management frame protection is not WPA3.
    let no_mfp = parse_rsne(&body(4, &[4], &[8], 0)).unwrap();
    assert_eq!(select(&no_mfp, false, JoinPolicy::WPA3_ONLY), Err(SelectError::BadProtection));
}

#[test]
fn unsupported_ciphers_and_akms_are_refused() {
    let tkip_group = parse_rsne(&body(2, &[4], &[2], 0)).unwrap();
    assert_eq!(tkip_group.group, CIPHER_TKIP);
    assert_eq!(select(&tkip_group, false, JoinPolicy::ANY), Err(SelectError::UnsupportedCipher));
    let tkip_only = parse_rsne(&body(2, &[2], &[2], 0)).unwrap();
    assert_eq!(select(&tkip_only, false, JoinPolicy::ANY), Err(SelectError::UnsupportedCipher));
    let enterprise = parse_rsne(&body(4, &[4], &[1], 0)).unwrap();
    assert_eq!(select(&enterprise, false, JoinPolicy::ANY), Err(SelectError::UnsupportedAkm));
    let mfpr_without_mfpc = parse_rsne(&body(4, &[4], &[2], 0x0040)).unwrap();
    assert_eq!(select(&mfpr_without_mfpc, false, JoinPolicy::ANY), Err(SelectError::BadProtection));
}
