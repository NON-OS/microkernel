// NONOS Operating System (AGPL-3.0-or-later)
//! Keys the chip never took: the engine holds the security CAM command, so
//! the driver says the key did not go in and the station runs CCMP itself.
//! On a chip that drops a protected frame no key opens, nothing from the
//! access point reaches the station then; on one that passes it up, the
//! station's own CCMP opens it. A frame for the chip to seal is refused
//! while the pairwise key's slot is not pointed at.

use super::station::{code, connect, netif, on_air, PASS, SSID};
use crate::ap_sim::AA;
use crate::soft::data_frames::from_ds;
use nonos_wifi_core::dot11::ccmp::encrypt;
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::rsn::TemporalKey;
use nonos_wifi_core::softmac::{ChipKey, Radio};

#[test]
fn keys_the_chip_never_took_leave_the_station_deaf_on_a_dropping_chip() {
    let (card, mut st) = on_air(6);
    card.0.borrow_mut().faults.sec_stall = true;
    assert_eq!(code(&connect(&mut st, SSID, PASS)), 0);
    let (sta, tk) = {
        let s = card.0.borrow();
        let Some(TemporalKey::Ccmp128(tk)) = s.air.as_ref().unwrap().ap.tk else { panic!() };
        assert!(s.sec.entries.is_empty(), "the chip holds no key");
        (s.cam.unwrap().0, tk)
    };
    assert_eq!(st.radio().unwrap().chip.vif.keys.valid, 0, "no slot left pointed at it");
    let mut hear = |pn, pass| {
        card.0.borrow_mut().faults.pass_unkeyed = pass;
        let f = encrypt(&from_ds(sta, &[0xaa, 0xaa, 3, 0, 0, 0, 8, 0, b'x']), pn, &tk).unwrap();
        card.0.borrow_mut().air.as_mut().unwrap().inbox.push_back(f);
        st.tick();
        netif(&mut st, 5, &[]).1
    };
    assert!(hear(1, false).is_empty(), "dropped by the chip, as the risk says");
    assert!(hear(2, true).ends_with(b"x"), "passed up: the station's CCMP opens it");
    let eth = [&AA[..], &sta, &[0x08, 0x00, 0x45]].concat();
    assert_eq!(netif(&mut st, 4, &eth).0, 0);
    let s = card.0.borrow();
    let last = s.sent.last().unwrap();
    assert_eq!((last.wd[8], last.frame.len()), (0, 24 + 8 + 9 + 8), "sealed by the station");
}

#[test]
fn a_frame_for_the_chip_goes_only_to_the_ap_under_its_pairwise_key() {
    let (_card, chip) = crate::model::rig::ready();
    let mut r = crate::radio::Station::new(chip);
    assert!(Radio::tune(&mut r, Band::Ghz2, 6));
    let frame = |fc: u8, to: [u8; 6]| {
        let mut f = vec![fc, 0x41, 0, 0];
        f.extend_from_slice(&to);
        f.resize(24 + 8 + 10, 0);
        f
    };
    r.chip.vif.bssid = AA;
    assert!(!r.set_key(&ChipKey::Pairwise(&[1; 16])), "no key before the association");
    assert!(!r.send_keyed(&frame(0x08, AA)), "no pairwise key");
    r.chip.vif.keys.set(0, 0);
    assert!(!r.send_keyed(&frame(0x08, [2; 6])), "not to the access point");
    assert!(!r.send_keyed(&frame(0xc0, AA)), "a management frame is the station's to seal");
    assert!(r.send_keyed(&frame(0x08, AA)));
}
