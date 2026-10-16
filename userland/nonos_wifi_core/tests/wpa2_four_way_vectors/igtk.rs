// NONOS Operating System (AGPL-3.0-or-later)

//! A PSK network requiring protection delivers the IGTK.

use nonos_wifi_core::eapol::parse::parse;
use nonos_wifi_core::rsn::build::station_rsne;
use nonos_wifi_core::rsn::{parse_rsne, select, JoinPolicy, Pmf};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::supplicant::{Config, State, Supplicant};

use super::vectors::{h, psk_ieee, AA, M1, SNONCE, SPA};

#[test]
fn a_psk_network_requiring_protection_delivers_the_igtk() {
    let ap_rsne = h("30140100000fac040100000fac040100000fac02cc00");
    let sel = select(&parse_rsne(&ap_rsne[2..]).unwrap(), false, JoinPolicy::PSK_ONLY).unwrap();
    assert_eq!((sel.akm, sel.pmf), (Akm::Psk, Pmf::Required));
    let own = station_rsne(&sel);
    assert_eq!(own.to_vec(), h("30140100000fac040100000fac040100000fac02c000"));

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
    assert_eq!(
        m2,
        h("0203007502010a0000000000000000000111111111111111111111111111111111111111
           111111111111111111111111110000000000000000000000000000000000000000000000
           0000000000000000006967e07e8bb52e0dbac95d3732c8cfc9001630140100000fac0401
           00000fac040100000fac02c000")
    );
    let m3 = h("020300b70213ca00100000000000000002202122232425262728292a2b2c2d2e2f303132
        333435363738393a3b3c3d3e3f0000000000000000000000000000000005070000000000
        0000000000000000000d2e0b7319f60ddf39fe1d6160e8857b0058a6400fb441baf425dd
        e4876d18810e1ebf1a7f8f72d30de0e4c1956837c95bf7abbeeaa45b7a92fde4b3c31dea
        cf86f9efca197eb166d35cfe13dcb3f6b95eb7e8d48ffb2a09f38b26979d956559460af0
        223f49ebfb4975");
    let m4 = s.step(&m3).reply.expect("message 4");
    assert_eq!(s.state(), State::Connected);
    let k = parse(&m4).unwrap();
    assert_eq!(k.key_info, 0x030a);
    assert_eq!(s.gtk_id(), 1);
    let (igtk, id, ipn) = s.igtk().expect("the IGTK KDE");
    assert_eq!(igtk.to_vec(), (0xc0..0xd0).collect::<Vec<u8>>());
    assert_eq!((id, ipn), (4, [0x2a, 0, 0, 0, 0, 0]));
}
