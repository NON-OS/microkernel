// NONOS Operating System (AGPL-3.0-or-later)
//! Traffic on a joined link: net_core's frame goes out through the data
//! queue under CCMP and the AP reads it; the AP's protected frame comes in
//! through the receive ring and net_core reads it in the clear.

use super::station::{code, connect, netif, on_air, Sta, PASS, SSID};
use crate::ap_sim::AA;
use crate::model::Card;
use crate::soft::data_frames::from_ds;
use nonos_wifi_core::dot11::ccmp::encrypt;
use nonos_wifi_core::rsn::TemporalKey;

fn joined() -> (Card, Sta) {
    let (card, mut st) = on_air(6);
    assert_eq!(code(&connect(&mut st, SSID, PASS)), 0);
    (card, st)
}

#[test]
fn net_cores_frames_go_both_ways_under_ccmp() {
    let (card, mut st) = joined();
    let sta = card.0.borrow().cam.unwrap().0;
    let mut eth = AA.to_vec();
    eth.extend_from_slice(&sta);
    eth.extend_from_slice(&[0x08, 0x00, 0x45, 0x00]);
    assert_eq!(netif(&mut st, 4, &eth).0, 0);
    let tk = {
        let s = card.0.borrow();
        let last = s.sent.last().unwrap();
        assert_eq!(last.ch, 0, "data on ACH0");
        assert_eq!(last.frame[1] & 0x40, 0x40, "protected on the air");
        let ap = &s.air.as_ref().unwrap().ap;
        assert_eq!(ap.got.last().unwrap(), &vec![0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x45, 0x00]);
        let Some(TemporalKey::Ccmp128(tk)) = ap.tk else { panic!("keyed for CCMP-128") };
        tk
    };
    let down = encrypt(&from_ds(sta, &[0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x99]), 1, &tk);
    card.0.borrow_mut().air.as_mut().unwrap().inbox.push_back(down.unwrap());
    st.tick();
    let (status, rx) = netif(&mut st, 5, &[]);
    assert_eq!(status, 0);
    assert_eq!(&rx[4..], &[sta.as_slice(), &AA, &[0x08, 0x00, 0x99]].concat()[..]);
    assert_eq!(card.0.borrow().violations, Vec::<String>::new());
}
