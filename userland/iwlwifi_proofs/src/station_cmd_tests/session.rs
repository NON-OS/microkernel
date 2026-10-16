// NONOS Operating System (AGPL-3.0-or-later)
//! Session protection, the access point's station entry, and a scan abort.

use crate::gen3::station::abort::scan_abort;
use crate::gen3::station::phy::{ACTION_ADD, ACTION_REMOVE};
use crate::gen3::station::session::{parse_notif, session_protection, Session};
use crate::gen3::station::sta::{sta_config, sta_remove};
use super::fixtures::{BSSID, image};

#[test]
fn session_protection_asks_for_900_ms_and_cancels_with_the_same_session() {
    let add = session_protection(ACTION_ADD, 900);
    assert_eq!(add.to_vec(), image(&[0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0x6E, 0x03, 0, 0], 24));
    let remove = session_protection(ACTION_REMOVE, 0);
    assert_eq!(remove.to_vec(), image(&[0, 0, 0, 0, 3, 0, 0, 0], 24));
    let notif = |link: u32, status: u32, start: u32, conf: u32| {
        [link, status, start, conf].iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<u8>>()
    };
    assert_eq!(parse_notif(&notif(0, 1, 1, 0)), Some(Session::Started));
    assert_eq!(parse_notif(&notif(0, 1, 0, 0)), Some(Session::Ended));
    assert_eq!(parse_notif(&notif(0, 0, 0, 0)), Some(Session::Refused));
    assert_eq!(parse_notif(&notif(1, 1, 1, 0)), None, "another link");
    assert_eq!(parse_notif(&notif(0, 1, 1, 4)), None, "another session");
    assert_eq!(parse_notif(&notif(0, 1, 1, 0)[..15]), None, "short");
}

#[test]
fn the_access_point_station_entry() {
    let mut want = vec![0u8; 96];
    want[8..14].copy_from_slice(&BSSID);
    want[16..22].copy_from_slice(&BSSID);
    want[36] = 1;
    assert_eq!(sta_config(0, BSSID, 0, true).to_vec(), want, "added, before authorization");
    want[28..30].copy_from_slice(&7u16.to_le_bytes());
    want[36] = 0;
    assert_eq!(sta_config(0, BSSID, 7, false).to_vec(), want, "authorized without protection");
    assert_eq!(sta_remove(), [0, 0, 0, 0]);
}

#[test]
fn a_scan_is_aborted_by_its_uid() {
    assert_eq!(scan_abort(), [0u8; 8]);
}
