// NONOS Operating System (AGPL-3.0-or-later)
//! W2c on the Realtek link: the Transition Disable bitmap message 3 carried
//! is what the link reports for the link reply, and none before a handshake.

use crate::ap_sim::{gtk_kde, message1, message3, AA, ANONCE, AP_RSNE_MIXED, GTK, SNONCE, SPA};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::supplicant::{Config, Supplicant};

use super::rig::link;

fn joined(extra: &[u8]) -> Supplicant {
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let mut sup = Supplicant::configure(&Config {
        pmk: key,
        aa: AA,
        spa: SPA,
        snonce: SNONCE,
        akm: Akm::Psk,
        own_rsne: &nonos_wifi_core::wpa::RSN_IE,
        own_rsnxe: None,
        ap_rsne: &AP_RSNE_MIXED,
        ap_rsnxe: None,
        pmf: false,
    });
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let ptk = Akm::Psk.derive_ptk(&key, &AA, &SPA, &ANONCE, &SNONCE);
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    kd.extend_from_slice(extra);
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd));
    sup
}

#[test]
fn the_link_reports_message3_s_transition_disable_bitmap() {
    for (extra, td) in [
        (&[0xDD, 5, 0x00, 0x0F, 0xAC, 0x20, 1][..], 1),
        (&[][..], 0),
        (&[0xDD, 4, 0x00, 0x0F, 0xAC, 0x20][..], 0),
    ] {
        let (mut link, _tx) = link();
        assert_eq!(link.transition_disable(), 0, "none before a handshake");
        link.associate(super::rig::AP, [0x5A; 16]);
        link.set_supplicant(joined(extra));
        assert_eq!(link.transition_disable(), td, "{extra:?}");
    }
}
