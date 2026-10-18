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

//! The scan list's bounds: the dBm to percent scale is clamped at both ends,
//! and malformed input (an overlong SSID, a buffer too small for a network,
//! more networks than the list holds) is dropped rather than truncated.

use crate::scan_list_tests::AP;
use nonos_wifi_core::scan_list::{signal_percent, ScanResults, MAX_RESULTS};

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
