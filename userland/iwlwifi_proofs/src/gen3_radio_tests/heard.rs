// NONOS Operating System (AGPL-3.0-or-later)
//! Beacons heard reach the scan list with their signal and security.

use nonos_wifi_core::scan_list::{ScanResults, FLAG_WPA2};

use crate::gen3::heard::hear;
use crate::gen3::rx_frame::RxFrame;

// A beacon for `ssid` from `bssid`, with an RSN element offering PSK.
fn beacon(bssid: [u8; 6], ssid: &[u8]) -> Vec<u8> {
    let mut f = vec![0x80, 0, 0, 0];
    f.extend_from_slice(&[0xFF; 6]);
    f.extend_from_slice(&bssid);
    f.extend_from_slice(&bssid);
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(&[0; 8]);
    f.extend_from_slice(&[0x64, 0, 0x11, 0x04]);
    f.push(0);
    f.push(ssid.len() as u8);
    f.extend_from_slice(ssid);
    f.extend_from_slice(&[48, 20, 1, 0, 0x00, 0x0F, 0xAC, 4, 1, 0, 0x00, 0x0F, 0xAC, 4, 1, 0, 0x00, 0x0F, 0xAC, 2, 0, 0]);
    f
}

#[test]
fn beacons_heard_reach_the_scan_list_with_signal_and_security() {
    let mut list = ScanResults::new();
    let ap = [0x02, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE];
    let f = RxFrame { frame: beacon(ap, b"Home"), channel: 6, signal: -60 };
    assert!(hear(&mut list, &f));
    let mut out = [0u8; 64];
    let n = list.encode(&mut out);
    assert_eq!(&out[..n], &[1, 80, 1 | FLAG_WPA2, 4, b'H', b'o', b'm', b'e']);
    // A probe request, and a beacon cut short, are not listed.
    let mut probe = beacon(ap, b"Other");
    probe[0] = 0x40;
    assert!(!hear(&mut list, &RxFrame { frame: probe, channel: 6, signal: -40 }));
    let short = beacon(ap, b"Home")[..30].to_vec();
    assert!(!hear(&mut list, &RxFrame { frame: short, channel: 6, signal: -40 }));
    assert_eq!(list.count(), 1);
}
