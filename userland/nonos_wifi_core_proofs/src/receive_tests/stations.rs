// NONOS Operating System (AGPL-3.0-or-later)

//! The stations the frames are fed to: associated, and joined through a handshake.

use nonos_wifi_core::station::{Ccmp, LinkStation};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::supplicant::{Config, Supplicant};

use super::frames::{AP, STA};
use super::vectors::tk;
use crate::ap_sim::{gtk_kde, message1, message3, AA, ANONCE, AP_RSNE_MIXED, GTK, SNONCE, SPA};

pub(super) fn station() -> LinkStation {
    let mut s = LinkStation::new(STA);
    s.associate(AP, Ccmp::Software { tk: tk() });
    s
}

// A station that finished a WPA2 join: its supplicant, the PTK the AP holds.
pub(super) fn joined() -> (LinkStation, [u8; 48]) {
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
    let mut m3 = message3(Akm::Psk, 2, &ANONCE, &ptk, &kd);
    m3[65] = 9; // Key RSC: the AP's group frames are past PN 9
    let m3 = {
        // Recompute the MIC over the edited RSC.
        let mic = nonos_wifi_core::eapol::mic::compute_mic(&ptk[..16], &m3);
        let mut f = m3;
        f[81..97].copy_from_slice(&mic);
        f
    };
    sup.step(&m3);
    let mut tkb = [0u8; 16];
    tkb.copy_from_slice(&ptk[32..48]);
    let mut s = LinkStation::new(STA);
    s.associate(AP, Ccmp::Software { tk: tkb });
    s.set_supplicant(sup);
    (s, ptk)
}
