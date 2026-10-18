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
//! heard at reaches the panel's encoding and a refresh replaces it. The
//! scale and the bounds are in `scan_list_bounds_tests`.

use nonos_wifi_core::scan_list::{ScanResults, FLAG_WPA2, FLAG_WPA3};

pub const AP: [u8; 6] = [0x02, 0, 0, 0, 0, 1];

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
