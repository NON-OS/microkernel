// NONOS Operating System (AGPL-3.0-or-later)
//! WPA3-Personal through the chip: SAE (hash-to-element) commit and
//! confirm on the management queue, association, the handshake under the
//! SAE PMK, and net_core's frame read by the AP under the new key.

use super::station::{code, connect, netif, Sta};
use crate::ap_sim::AA;
use crate::model::{rig, Card};
use crate::radio::Station;
use crate::sae_air::{SaeAir, PASS, SSID};
use crate::station::CapsuleEnv;
use nonos_wifi_core::softmac::SoftStation;

fn on_wpa3_air(channel: u8) -> (Card, Sta) {
    let (card, chip) = rig::ready();
    card.0.borrow_mut().sae = Some(SaeAir::new(channel));
    (card, SoftStation::new(Some(Station::new(chip)), CapsuleEnv, 0))
}

#[test]
fn joins_wpa3_sae_and_carries_traffic() {
    let (card, mut st) = on_wpa3_air(44);
    let r = connect(&mut st, SSID, PASS);
    assert_eq!(code(&r), 0, "joined: {r:?}");
    {
        let s = card.0.borrow();
        assert_eq!(s.violations, Vec::<String>::new());
        let sae = s.sae.as_ref().unwrap();
        assert!(sae.ap.tk.is_some(), "message 4 installed the key");
        let auths = s.sent.iter().filter(|f| f.ch == 8 && f.frame[0] == 0xb0).count();
        assert_eq!(auths, 2, "SAE commit and confirm on CH8");
    }
    let sta = card.0.borrow().cam.unwrap().0;
    let mut eth = AA.to_vec();
    eth.extend_from_slice(&sta);
    eth.extend_from_slice(&[0x08, 0x00, 0x45, 0x00]);
    assert_eq!(netif(&mut st, 4, &eth).0, 0);
    let s = card.0.borrow();
    let got = &s.sae.as_ref().unwrap().ap.got;
    assert_eq!(got.last().unwrap(), &vec![0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x45, 0x00]);
}

#[test]
fn a_wrong_wpa3_password_never_gets_a_key() {
    let (card, mut st) = on_wpa3_air(1);
    let r = connect(&mut st, SSID, b"wrong horse battery");
    assert_ne!(code(&r), 0);
    assert!(card.0.borrow().sae.as_ref().unwrap().ap.tk.is_none());
}
