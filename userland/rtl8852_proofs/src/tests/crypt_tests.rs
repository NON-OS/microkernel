// NONOS Operating System (AGPL-3.0-or-later)
//! CCMP-128 in the chip on the model, which drops a protected frame no key
//! slot opens. A WPA2 join puts the TK in security CAM entry 0, the GTK in
//! 2, the address CAM pointing at both; data goes out with the station's
//! CCMP header for the chip to seal and comes in decrypted, replay-checked
//! on its header; a forged MIC goes no further; leaving takes the slots.

use super::station::{code, connect, control, netif, on_air, Sta, PASS, SSID};
use crate::ap_sim::{AA, GTK};
use crate::model::Card;
use crate::soft::data_frames::from_ds;
use nonos_wifi_core::dot11::ccmp::encrypt;
use nonos_wifi_core::rsn::TemporalKey;

pub fn joined() -> (Card, Sta, [u8; 16]) {
    let (card, mut st) = on_air(6);
    assert_eq!(code(&connect(&mut st, SSID, PASS)), 0);
    let Some(TemporalKey::Ccmp128(tk)) = card.0.borrow().air.as_ref().unwrap().ap.tk else {
        panic!("the AP is keyed for CCMP-128")
    };
    (card, st, tk)
}

/// `tag` from the AP under `key` at `pn`, MIC flipped by `forge`; what came up.
pub fn hear(card: &Card, st: &mut Sta, tag: &[u8], pn: u64, key: &[u8; 16], forge: u8) -> Vec<u8> {
    let sta = card.0.borrow().cam.unwrap().0;
    let f = from_ds(sta, &[&[0xaa, 0xaa, 3, 0, 0, 0, 8, 0][..], tag].concat());
    let mut f = encrypt(&f, pn, key).unwrap();
    *f.last_mut().unwrap() ^= forge;
    card.0.borrow_mut().air.as_mut().unwrap().inbox.push_back(f);
    st.tick();
    netif(st, 5, &[]).1
}

#[test]
fn a_wpa2_join_keys_the_chip_and_data_goes_both_ways_through_it() {
    let (card, mut st, tk) = joined();
    {
        let s = card.0.borrow();
        assert_eq!((s.sec.entries[&0], s.sec.entries[&2]), ((6, tk), (6, GTK)), "TK, GTK");
        assert_eq!((s.sec.slots[0], s.sec.slots[2]), (Some((0, 0)), Some((2, 1))));
    }
    let sta = card.0.borrow().cam.unwrap().0;
    let eth = [&AA[..], &sta, &[0x08, 0x00, 0x45, 0x00]].concat();
    assert_eq!(netif(&mut st, 4, &eth).0, 0);
    {
        let s = card.0.borrow();
        let last = s.sent.last().unwrap();
        assert_eq!(last.wd[8], 6 << 9 | 1 << 8, "CCMP-128, HW_ENC, entry 0");
        assert_eq!((last.frame.len(), last.frame[27] & 0x20), (24 + 8 + 10, 0x20), "no MIC");
        let ap = &s.air.as_ref().unwrap().ap;
        assert_eq!(ap.got.last().unwrap(), &vec![0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x45, 0x00]);
    }
    assert_eq!(netif(&mut st, 4, &eth).0, 0);
    let pns: Vec<_> = card.0.borrow().sent.iter().rev().take(2).map(|t| t.frame[24]).collect();
    assert_eq!(pns[0], pns[1] + 1, "the station numbers each frame it hands the chip");
    assert!(hear(&card, &mut st, b"in", 1, &tk, 0).ends_with(b"in"), "decrypted by the chip");
    assert!(card.0.borrow().violations.is_empty());
}

#[test]
fn a_replay_or_a_forged_mic_never_reaches_the_stack() {
    let (card, mut st, tk) = joined();
    assert!(hear(&card, &mut st, b"once", 5, &tk, 0).ends_with(b"once"));
    assert!(hear(&card, &mut st, b"twice", 5, &tk, 0).is_empty(), "the same packet number");
    assert!(hear(&card, &mut st, b"forged", 6, &tk, 1).is_empty(), "ICV_ERR");
    assert!(hear(&card, &mut st, b"later", 7, &tk, 0).ends_with(b"later"));
}

#[test]
fn leaving_takes_every_key_slot_away() {
    let (card, mut st, _) = joined();
    control(&mut st, 2, &[]);
    assert_eq!(card.0.borrow().sec.slots, [None; 7]);
}
