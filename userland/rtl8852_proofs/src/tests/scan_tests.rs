// NONOS Operating System (AGPL-3.0-or-later)
//! Scanning (`rtw89_hw_scan`'s software path, `rtw89_core_scan_start` and
//! `_complete`): EDCCA raised to its maximum and the RX filter opened while
//! the station listens, both given back as they were when it settles on a
//! network; the panel's scan list names the AP heard through the chip; and
//! power save stays off throughout (no PS command ever reaches the firmware).

use super::station::{code, connect, control, on_air, PASS, SSID};
use crate::model::rig;
use crate::radio::Station;
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::Radio;

const EDCCA: u32 = 0x1_4884;
const FLTR: u32 = 0xce20;

#[test]
fn a_scan_opens_the_filter_and_raises_edcca_then_gives_both_back() {
    let (card, chip) = rig::ready();
    let mut sta = Station::new(chip);
    assert!(sta.tune(Band::Ghz2, 6));
    let (edcca, fltr) = {
        let s = card.0.borrow();
        (s.word(EDCCA), s.word(FLTR))
    };
    sta.point(None);
    {
        let s = card.0.borrow();
        assert_eq!(s.word(EDCCA) & 0xff00_ffff, 0xf900_f9f9, "EDCCA at its maximum");
        assert_eq!(s.word(FLTR) & 0x86, 0, "beacon check, broadcast and A1 match off");
    }
    sta.point(None);
    sta.point(Some(crate::ap_sim::AA));
    let s = card.0.borrow();
    assert_eq!(s.word(EDCCA), edcca, "a second scan start kept the first backup");
    assert_eq!(s.word(FLTR), fltr);
    assert_eq!(s.cam.unwrap().1, crate::ap_sim::AA);
    assert_eq!(sta.counts.cmd_fail, 0);
}

#[test]
fn the_scan_list_names_the_ap_heard_through_the_chip() {
    let (card, mut st) = on_air(11);
    let mut names = Vec::new();
    for _ in 0..400 {
        crate::clock::idle_ms(100); // the serve loop's wait for a request
        st.tick();
        let r = control(&mut st, 3, &[]);
        if r[12] > 0 {
            names.push(r[16..16 + r[15] as usize].to_vec());
            break;
        }
    }
    assert_eq!(names, vec![SSID.to_vec()]);
    assert!(card.0.borrow().sent.is_empty(), "the scan only listens");
}

#[test]
fn power_save_is_never_asked_of_the_firmware() {
    let (card, mut st) = on_air(6);
    assert_eq!(code(&connect(&mut st, SSID, PASS)), 0);
    for _ in 0..50 {
        st.tick();
    }
    let s = card.0.borrow();
    assert!(!s.h2c.iter().any(|c| (c.cat, c.class) == (1, 1)), "no H2C_CL_MAC_PS command");
    assert!(s.powered);
}
