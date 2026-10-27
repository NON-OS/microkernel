// NONOS Operating System (AGPL-3.0-or-later)
//! What a person at the serial port sees, and what the chip is told: every
//! console line is written whole, ends its line and fits; none carries an
//! address or the network's name; and the efuse's MAC address reaches no
//! register, command or frame.

use super::station::{code, connect, on_air, PASS, SSID};
use crate::model::efuse_img::EFUSE_MAC;
use crate::status::LINE_MAX;

fn holds(hay: &[u8], needle: &[u8]) -> bool {
    hay.windows(needle.len()).any(|w| w == needle)
}

#[test]
fn every_line_is_whole_and_names_no_address_or_network() {
    let _ = nonos_libc::take_lines();
    let (card, mut st) = on_air(6);
    assert_eq!(code(&connect(&mut st, SSID, PASS)), 0);
    let lines = nonos_libc::take_lines();
    assert!(lines.len() >= 2, "bring-up reports itself");
    let s = card.0.borrow();
    let (sma, bssid) = s.cam.unwrap();
    for l in &lines {
        assert!(l.ends_with(b"\n") && l.len() <= LINE_MAX, "{l:?}");
        assert_eq!(l.iter().filter(|&&b| b == b'\n').count(), 1, "one line per write");
        for secret in [&sma[..], &bssid[..], &EFUSE_MAC[..], SSID] {
            assert!(!holds(l, secret), "{l:?}");
        }
        let text = String::from_utf8_lossy(l).to_lowercase();
        assert!(!text.contains("thisisassid") && !text.contains(":"), "{text}");
    }
}

#[test]
fn the_first_address_is_drawn_local_and_unicast() {
    for _ in 0..64 {
        let a = crate::station::draw().unwrap();
        assert_eq!(a[0] & 0x03, 0x02, "{a:02x?}");
    }
    let (card, chip) = crate::model::rig::ready();
    let first = card.0.borrow().h2c.iter().find(|c| c.is(1, 6, 0)).unwrap().body.clone();
    assert_eq!(&first[16..22], &chip.vif.sma, "the station interface's CAM entry");
}

#[test]
fn the_efuse_mac_reaches_no_register_command_or_frame() {
    let (card, mut st) = on_air(6);
    assert_eq!(code(&connect(&mut st, SSID, PASS)), 0);
    let s = card.0.borrow();
    let (sma, _) = s.cam.unwrap();
    assert_ne!(sma, EFUSE_MAC);
    assert_eq!(sma[0] & 0x03, 0x02, "locally administered, unicast");
    for c in &s.h2c {
        assert!(!holds(&c.body, &EFUSE_MAC), "{c:?}");
    }
    for f in &s.sent {
        assert!(!holds(&f.frame, &EFUSE_MAC));
    }
    let words: Vec<u8> = s.log.iter().flat_map(|w| w.1.to_le_bytes()).collect();
    assert!(!holds(&words, &EFUSE_MAC[..4]) && !holds(&words, &EFUSE_MAC[2..]));
}
