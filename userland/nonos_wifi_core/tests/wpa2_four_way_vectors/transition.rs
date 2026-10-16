// NONOS Operating System (AGPL-3.0-or-later)

//! The PTK in either address order, and a transition network joined with PSK.

use nonos_wifi_core::rsn::build::station_rsne;
use nonos_wifi_core::rsn::{parse_rsne, select, JoinPolicy, Pmf};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::ptk;
use nonos_wifi_core::wpa::supplicant::{Config, State, Supplicant};
use nonos_wifi_core::wpa::RSN_IE;

use super::vectors::{
    anonce, h, psk_ieee, AA, AP_RSNE_TRANSITION, AP_RSNXE, M1, M2_TRANSITION, M3_TRANSITION,
    M4_TRANSITION, PTK, SNONCE, SPA,
};

#[test]
fn the_ptk_matches_the_reference_in_either_address_order() {
    let pmk = psk_ieee();
    assert_eq!(ptk(&pmk, &AA, &SPA, &anonce(), &SNONCE).to_vec(), h(PTK));
    // The same PTK from the authenticator's side of the argument list.
    assert_eq!(Akm::Psk.derive_ptk(&pmk, &AA, &SPA, &anonce(), &SNONCE).to_vec(), h(PTK));
}

#[test]
fn a_transition_network_joined_with_psk_completes_against_the_reference() {
    let ap_rsne = h(AP_RSNE_TRANSITION);
    let ap_rsnxe = h(AP_RSNXE);
    let sel = select(&parse_rsne(&ap_rsne[2..]).unwrap(), true, JoinPolicy::PSK_ONLY).unwrap();
    assert_eq!((sel.akm, sel.pmf), (Akm::Psk, Pmf::Off));
    let own = station_rsne(&sel);
    assert_eq!(own, RSN_IE, "PSK without protection sends the plain WPA2 RSNE");

    let mut s = Supplicant::configure(&Config {
        pmk: psk_ieee(),
        aa: AA,
        spa: SPA,
        snonce: SNONCE,
        akm: sel.akm,
        own_rsne: &own,
        own_rsnxe: None,
        ap_rsne: &ap_rsne,
        ap_rsnxe: Some(&ap_rsnxe),
        pmf: false,
    });
    let m2 = s.step(&h(M1)).reply.expect("message 2");
    assert_eq!(
        m2,
        h(M2_TRANSITION),
        "message 2: header length, Key Information 0x010a, key length 0, MIC, RSNE"
    );

    // Message 3, with four bytes of trailing junk (an FCS the driver kept).
    let mut m3 = h(M3_TRANSITION);
    m3.extend_from_slice(&[0xde, 0xad, 0xbe, 0xef]);
    let m4 = s.step(&m3).reply.expect("message 4");
    assert_eq!(s.state(), State::Connected);
    assert_eq!(
        m4,
        h(M4_TRANSITION),
        "message 4: Key Information 0x030a, replay 2, zero nonce, MIC, no key data"
    );
    assert_eq!(s.tk().to_vec(), h(PTK)[32..].to_vec());
    assert_eq!(s.gtk().to_vec(), (0xa0..0xb0).collect::<Vec<u8>>());
    assert_eq!((s.gtk_id(), s.gtk_rsc()), (2, 0x0705));
    assert!(s.igtk().is_none());
}
