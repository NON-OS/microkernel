// NONOS Operating System (AGPL-3.0-or-later)

//! A PSK network requiring protection delivers the IGTK.

use nonos_wifi_core::eapol::parse::parse;
use nonos_wifi_core::rsn::build::station_rsne;
use nonos_wifi_core::rsn::{parse_rsne, select, JoinPolicy, Pmf};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::supplicant::{Config, State, Supplicant};

use super::protected::{AP_RSNE_PROTECTED, M2_PROTECTED, M3_PROTECTED, OWN_RSNE_PROTECTED};
use super::vectors::{h, psk_ieee, AA, M1, SNONCE, SPA};

#[test]
fn a_psk_network_requiring_protection_delivers_the_igtk() {
    let ap_rsne = h(AP_RSNE_PROTECTED);
    let sel = select(&parse_rsne(&ap_rsne[2..]).unwrap(), false, JoinPolicy::ANY).unwrap();
    assert_eq!((sel.akm, sel.pmf), (Akm::Psk, Pmf::Required));
    let own = station_rsne(&sel);
    assert_eq!(own.to_vec(), h(OWN_RSNE_PROTECTED));

    let mut s = Supplicant::configure(&Config {
        pmk: psk_ieee(),
        aa: AA,
        spa: SPA,
        snonce: SNONCE,
        akm: sel.akm,
        own_rsne: &own,
        own_rsnxe: None,
        ap_rsne: &ap_rsne,
        ap_rsnxe: None,
        pmf: true,
    });
    let m2 = s.step(&h(M1)).reply.expect("message 2");
    assert_eq!(m2, h(M2_PROTECTED));
    let m3 = h(M3_PROTECTED);
    let m4 = s.step(&m3).reply.expect("message 4");
    assert_eq!(s.state(), State::Connected);
    let k = parse(&m4).unwrap();
    assert_eq!(k.key_info, 0x030a);
    assert_eq!(s.gtk_id(), 1);
    let (igtk, id, ipn) = s.igtk().expect("the IGTK KDE");
    assert_eq!(igtk.to_vec(), (0xc0..0xd0).collect::<Vec<u8>>());
    assert_eq!((id, ipn), (4, [0x2a, 0, 0, 0, 0, 0]));
}
