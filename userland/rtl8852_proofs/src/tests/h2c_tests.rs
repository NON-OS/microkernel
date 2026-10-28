// NONOS Operating System (AGPL-3.0-or-later)
//! The firmware commands, as the firmware takes them: numbered from 0 after
//! each download and one apart, a receive ack asked on every fourth
//! (`rtw89_h2c_pkt_set_hdr`), the station interface's in
//! `rtw89_ops_add_interface`'s order, and association's in
//! `rtw89_station_mode_sta_assoc`'s.

use super::station::{code, connect, on_air, PASS, SSID};
use crate::model::cpu::H2c;
use crate::model::rig;

fn ids(h: &[H2c]) -> Vec<(u8, u8, u8)> {
    h.iter().map(|c| (c.cat, c.class, c.func)).collect()
}

#[test]
fn commands_are_numbered_one_apart_with_a_receive_ack_every_fourth() {
    let (card, mut st) = on_air(6);
    assert_eq!(code(&connect(&mut st, SSID, PASS)), 0);
    let s = card.0.borrow();
    // The download resets the count (`rtw89_fw_download`), and the probe's
    // firmware took no command, so the list starts at mac_init's first.
    assert_eq!(s.h2c[0].seq, 0);
    assert!(s.h2c.len() > 12, "reaches 12, a fourth that is not an eighth");
    for p in s.h2c.windows(2) {
        assert_eq!(p[1].seq, p[0].seq.wrapping_add(1));
    }
    // The MAC ID unpause asks its own ack (`rtw89_fw_h2c_macid_pause`).
    for c in s.h2c.iter().filter(|c| !c.is(1, 9, 8)) {
        assert_eq!(c.rack, c.seq % 4 == 0, "{c:?}");
    }
    assert!(s.h2c.iter().filter(|c| c.is(1, 9, 8)).all(|c| c.rack));
}

#[test]
fn the_station_interface_is_added_in_rtw89s_order() {
    let (card, _chip) = rig::ready();
    let s = card.0.borrow();
    let tail = &ids(&s.h2c)[s.h2c.len() - 5..];
    let want = [(1, 9, 8), (1, 8, 4), (1, 8, 0), (1, 6, 0), (1, 5, 2)];
    assert_eq!(tail, want, "macid unpause, role create, join (no link), CAM, CMAC table");
    let join = &s.h2c[s.h2c.len() - 3].body;
    let w = u32::from_le_bytes(join[..4].try_into().unwrap());
    assert_eq!((w & 0x100, (w >> 24) & 3), (0x100, 0), "disconnected, no link");
}

#[test]
fn association_tells_the_firmware_in_rtw89s_order() {
    let (card, mut st) = on_air(6);
    let before = card.0.borrow().h2c.len();
    assert_eq!(code(&connect(&mut st, SSID, PASS)), 0);
    let s = card.0.borrow();
    let after = ids(&s.h2c[before..]);
    let at = after.iter().position(|&c| c == (2, 1, 0)).expect("the rate adaptation command");
    assert_eq!(&after[at - 3..=at], &[(1, 5, 2), (1, 8, 0), (1, 6, 0), (2, 1, 0)]);
    let join = &s.h2c[before + at - 2].body;
    let w = u32::from_le_bytes(join[..4].try_into().unwrap());
    assert_eq!((w & 0x100, (w >> 24) & 3), (0, 2), "connected, infrastructure");
    let cam = &s.h2c[before + at - 1].body;
    let aid = u32::from_le_bytes(cam[36..40].try_into().unwrap()) & 0xfff;
    assert_ne!(aid, 0, "the association ID in the CAM");
    let (_, bssid) = s.cam.unwrap();
    assert_eq!(bssid, crate::ap_sim::AA, "the AP's BSSID in the CAM");
}
