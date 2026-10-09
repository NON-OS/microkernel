// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The shared scan list as both drivers use it: the signal each network was
//! heard at reaches the panel's encoding, a refresh replaces it, the dBm to
//! percent scale is clamped at both ends, and malformed input (an overlong
//! SSID, a buffer too small for a network) is dropped rather than truncated.

use nonos_wifi_core::scan_list::{signal_percent, ScanResults, FLAG_WPA2, FLAG_WPA3, MAX_RESULTS};

const AP: [u8; 6] = [0x02, 0, 0, 0, 0, 1];

#[test]
fn heard_signal_reaches_the_encoding() {
    let mut r = ScanResults::new();
    r.heard(AP, b"home", FLAG_WPA2 | FLAG_WPA3 | 1, 73);
    let mut out = [0u8; 64];
    let n = r.encode(&mut out);
    assert_eq!(&out[..n], &[1, 73, 0x07, 4, b'h', b'o', b'm', b'e']);
}

#[test]
fn a_refresh_replaces_signal_and_flags() {
    let mut r = ScanResults::new();
    r.heard(AP, b"home", FLAG_WPA2 | 1, 20);
    r.heard(AP, b"home", FLAG_WPA3 | 1, 90);
    assert_eq!(r.count(), 1);
    let mut out = [0u8; 64];
    let n = r.encode(&mut out);
    assert_eq!(&out[..n], &[1, 90, FLAG_WPA3 | 1, 4, b'h', b'o', b'm', b'e']);
}

#[test]
fn add_bss_records_no_signal() {
    let mut r = ScanResults::new();
    r.add_bss(AP, b"x", FLAG_WPA2 | 1);
    let mut out = [0u8; 8];
    let n = r.encode(&mut out);
    assert_eq!(&out[..n], &[1, 0, FLAG_WPA2 | 1, 1, b'x']);
}

#[test]
fn signal_percent_is_clamped_and_zero_means_unmeasured() {
    assert_eq!(signal_percent(-100), 0);
    assert_eq!(signal_percent(-128), 0);
    assert_eq!(signal_percent(-75), 50);
    assert_eq!(signal_percent(-50), 100);
    assert_eq!(signal_percent(-1), 100);
    assert_eq!(signal_percent(0), 0);
    assert_eq!(signal_percent(127), 0);
}

#[test]
fn an_overlong_ssid_is_not_listed() {
    let mut r = ScanResults::new();
    r.heard(AP, &[b'a'; 33], 1, 50);
    r.heard(AP, &[], 1, 50);
    assert_eq!(r.count(), 0);
    r.heard(AP, &[b'a'; 32], 1, 50);
    assert_eq!(r.count(), 1);
}

#[test]
fn a_network_that_does_not_fit_is_dropped_whole() {
    let mut r = ScanResults::new();
    r.heard(AP, b"first", 1, 10);
    r.heard([0x02, 0, 0, 0, 0, 2], b"second", 1, 20);
    // Room for the count and the first network (3 + 5) plus two bytes.
    let mut out = [0xEEu8; 11];
    let n = r.encode(&mut out);
    assert_eq!(n, 9);
    assert_eq!(out[0], 1);
    assert_eq!(&out[9..], &[0xEE, 0xEE]);
    assert_eq!(r.encode(&mut []), 0);
}

#[test]
fn the_list_is_capped() {
    let mut r = ScanResults::new();
    for i in 0..(MAX_RESULTS as u8 + 4) {
        r.heard([0x02, 0, 0, 0, 1, i], b"n", 1, i);
    }
    assert_eq!(r.count(), MAX_RESULTS);
}
