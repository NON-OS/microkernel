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
//! Vol 12's procmon table (p. 392), column by column, and the columns it
//! does not have.

use nonos_intel_xelp_display::phy::procmon::for_dw3;

/// PORT_COMP_DW3 with voltage 25:24 and process 28:26 (Vol 2c p. 899).
fn dw3(voltage: u32, process: u32) -> u32 {
    (process << 26) | (voltage << 24) | 0x00FF_FFFF
}

#[test]
fn every_column() {
    // (voltage code, process code, DW1 23:16, DW9, DW10); DW1 7:0 is 0.
    let cols = [
        (0, 0, 0x00, 0x62AB67BB, 0x51914F96),
        (1, 0, 0x00, 0x86E172C7, 0x77CA5EAB),
        (1, 1, 0x00, 0x93F87FE1, 0x8AE871C5),
        (2, 0, 0x00, 0x98FA82DD, 0x89E46DC1),
        (2, 1, 0x44, 0x9A00AB25, 0x8AE38FF1),
    ];
    for (v, p, hi, d9, d10) in cols {
        let pm = for_dw3(dw3(v, p)).unwrap();
        assert_eq!((pm.dw1_low, pm.dw1_high, pm.dw9, pm.dw10), (0, hi, d9, d10), "{v} {p}");
    }
}

#[test]
fn columns_the_table_lacks() {
    // 0.85 V dot-1 is N/A; dot-4 (010b) and the reserved codes are unlisted;
    // voltage code 11b is reserved.
    for (v, p) in [(0, 1), (0, 2), (1, 2), (2, 2), (3, 0), (3, 1), (1, 3), (2, 7)] {
        assert_eq!(for_dw3(dw3(v, p)), None, "{v} {p}");
    }
}
