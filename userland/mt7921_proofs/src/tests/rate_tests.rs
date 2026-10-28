// NONOS Operating System (AGPL-3.0-or-later)

//! The rate each frame goes at (`mac::pick`, after
//! `mt76_connac2_mac_tx_rate_val`): a frame to the access point at the
//! lowest basic rate it named, its data at the fastest, its EAPOL at the
//! lowest, anything else at the band's lowest; the basic rates read from
//! its beacon before authentication. On the model's air an access point
//! with 802.11b off hears no CCK, and the station still joins it.

use crate::mac::pick::pick;
use crate::model::net::{on_air, SSID};
use crate::model::phy::G_ONLY;
use crate::model::wire::{code, connect, netif, NET_CORE};

fn frame(fc: [u8; 2], body: &[u8]) -> Vec<u8> {
    let mut f = vec![0u8; 24];
    f[..2].copy_from_slice(&fc);
    f.extend_from_slice(body);
    f
}

const EAPOL: [u8; 10] = [0xaa, 0xaa, 3, 0, 0, 0, 0x88, 0x8e, 2, 3];
const IP: [u8; 10] = [0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x45, 0];

#[test]
fn frames_to_the_access_point_take_its_basic_rates() {
    let bss = Some((12, 48));
    let auth = frame([0xb0, 0], &[0; 6]);
    assert_eq!(pick(&auth, true, bss, false), 0x4b, "6 Mb/s OFDM, its lowest basic");
    assert_eq!(pick(&auth, true, None, false), 0, "1 Mb/s when none is known");
    assert_eq!(pick(&auth, false, bss, false), 0, "a probe to everyone at the band's lowest");
    assert_eq!(pick(&auth, true, None, true), 0x4b, "never CCK on 5 GHz");
    assert_eq!(pick(&frame([0x08, 0x01], &IP), true, bss, false), 0x49, "data at 24 Mb/s");
    assert_eq!(pick(&frame([0x88, 0x01], &[0, 0, 0xaa]), true, bss, false), 0x49, "QoS data");
    assert_eq!(pick(&frame([0x08, 0x01], &EAPOL), true, bss, false), 0x4b, "EAPOL lowest");
    let qos_eapol = frame([0x88, 0x01], &[[7, 0].as_slice(), &EAPOL].concat());
    assert_eq!(pick(&qos_eapol, true, bss, false), 0x4b, "past the QoS field too");
    let sealed = frame([0x08, 0x41], &EAPOL);
    assert_eq!(pick(&sealed, true, bss, false), 0x49, "a protected frame is data");
}

#[test]
fn an_access_point_without_cck_is_joined_at_ofdm_rates() {
    let (mut st, c) = on_air();
    c.borrow_mut().g_only = true;
    let r = connect(&mut st, SSID, b"correct horse battery");
    assert_eq!(code(&r), 0, "joined: {r:?}");
    assert_eq!(netif(&mut st, NET_CORE, 2, &[]).1, [1], "the link is up");
    let chip = c.borrow();
    let ap = chip.air.as_ref().unwrap().ap.bssid;
    let to_ap: Vec<u32> =
        chip.sent.iter().filter(|t| t.frame[4..10] == ap).map(|t| t.rate()).collect();
    assert!(!to_ap.is_empty() && to_ap.iter().all(|r| r >> 6 == 1), "all OFDM: {to_ap:x?}");
    assert!(to_ap.contains(&0x4b), "management at 6 Mb/s, its lowest basic rate");
    assert!(chip.air.as_ref().unwrap().ap.tk.is_some());
    assert_eq!(G_ONLY[0], 0x8c);
}
