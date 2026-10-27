// NONOS Operating System (AGPL-3.0-or-later)
//! A WPA3-Personal join (SAE, protected management frames required) on the
//! model: the pairwise key, the group key and the IGTK go into the chip,
//! the IGTK in a BIP entry; data from the access point comes up through
//! it; its protected deauthentication ends the link where an unprotected
//! one does not; and the station's own goes out sealed by the station, as
//! rtw89 leaves management frames to mac80211 on the 8852B
//! (`hw_mgmt_tx_encrypt` false: `IEEE80211_KEY_FLAG_SW_MGMT_TX`).

use super::station::{code, connect, control, netif, Sta};
use crate::ap_sim::{AA, GTK};
use crate::model::{rig, Card};
use crate::radio::Station;
use crate::sae_air::{SaeAir, PASS, SSID};
use crate::soft::data_frames::from_ds;
use crate::soft::frames::mgmt_from;
use crate::station::CapsuleEnv;
use nonos_wifi_core::dot11::ccmp::{decrypt, encrypt};
use nonos_wifi_core::softmac::SoftStation;

fn wpa3() -> (Card, Sta, [u8; 16], [u8; 6]) {
    let (card, chip) = rig::ready();
    card.0.borrow_mut().sae = Some(SaeAir::new(36));
    let mut st = SoftStation::new(Some(Station::new(chip)), CapsuleEnv, 0);
    assert_eq!(code(&connect(&mut st, SSID, PASS)), 0);
    let tk = card.0.borrow().sae.as_ref().unwrap().ap.tk.unwrap();
    let sta = card.0.borrow().cam.unwrap().0;
    (card, st, tk, sta)
}

fn hear(card: &Card, st: &mut Sta, f: Vec<u8>) -> Vec<u8> {
    card.0.borrow_mut().sae.as_mut().unwrap().inbox.push_back(f);
    st.tick();
    netif(st, 5, &[]).1
}

fn up(st: &mut Sta) -> bool {
    netif(st, 2, &[]).1 == [1]
}

#[test]
fn an_sae_join_keys_the_chip_with_the_igtk_and_data_comes_up() {
    let (card, mut st, tk, sta) = wpa3();
    {
        let e = &card.0.borrow().sec.entries;
        assert_eq!((e[&0], e[&2], e[&5]), ((6, tk), (6, GTK), (10, [0x77; 16])), "TK GTK IGTK");
    }
    let f = encrypt(&from_ds(sta, &[0xaa, 0xaa, 3, 0, 0, 0, 8, 0, b'w']), 3, &tk).unwrap();
    assert!(hear(&card, &mut st, f).ends_with(b"w"));
}

#[test]
fn only_a_protected_deauthentication_ends_the_link() {
    let (card, mut st, tk, sta) = wpa3();
    hear(&card, &mut st, mgmt_from(AA, 12, sta, &[7, 0]));
    assert!(up(&mut st), "an unprotected one is not the access point's word");
    hear(&card, &mut st, encrypt(&mgmt_from(AA, 12, sta, &[7, 0]), 4, &tk).unwrap());
    assert!(!up(&mut st), "the chip opened it and the station left");
}

#[test]
fn the_stations_own_deauthentication_is_sealed_by_the_station() {
    let (card, mut st, tk, _) = wpa3();
    control(&mut st, 2, &[]);
    let s = card.0.borrow();
    let d = s.sent.iter().rev().find(|t| t.frame[0] == 0xc0).expect("a deauthentication");
    assert_eq!((d.ch, d.wd[8]), (8, 0), "on the management queue, nothing asked of the chip");
    assert_eq!(decrypt(d.air.as_ref().unwrap(), &tk).as_deref(), Some(&[3u8, 0][..]));
}
