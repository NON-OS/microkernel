// NONOS Operating System (AGPL-3.0-or-later)
//! A full scan list fits the reply.

use nonos_wifi_core::scan_list::ScanResults;

use crate::control::{answer, Heard, View, REPLY_MAX};
use super::control::request;

#[test]
fn a_full_scan_list_fits_the_reply() {
    let mut list = ScanResults::new();
    for i in 0..20u8 {
        list.heard([2, 0, 0, 0, 1, i], &[b'n'; 32], 1, i);
    }
    let view = View {
        stage: 0,
        step: 0,
        detail: 0,
        heard: Some(Heard { list: &list, sweeps: 1, stalls: 0, frames: 0, beacons: 0 }),
    };
    let mut out = vec![0u8; REPLY_MAX];
    let n = answer(&request(3, 1), &view, &mut out).unwrap();
    assert_eq!(n, REPLY_MAX, "sixteen networks at the longest SSID");
    assert_eq!(out[22], 16);
}
