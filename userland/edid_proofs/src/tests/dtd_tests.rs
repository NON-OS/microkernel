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

//! Detailed timings: every field of Table 3.21 through its packed bits.

use crate::base::dtd::Detailed;
use crate::build::{dtd, dtd_1080p};

#[test]
fn the_1080p_descriptor_reads_back_field_by_field() {
    let d = Detailed::decode(&dtd_1080p()).unwrap();
    assert_eq!(
        (d.pixel_khz, d.h_active, d.h_blank, d.h_front, d.h_sync),
        (148_500, 1920, 280, 88, 44)
    );
    assert_eq!((d.v_active, d.v_blank, d.v_front, d.v_sync), (1080, 45, 4, 5));
    assert!(d.h_positive && d.v_positive && !d.interlaced);
    assert_eq!((d.width_mm, d.height_mm), (531, 299));
    assert_eq!((d.h_total(), d.v_total()), (2200, 1125));
    assert_eq!(d.refresh_mhz(), 60_000);
}

#[test]
fn every_high_bit_lands_where_table_3_21_puts_it() {
    // Largest values each field holds: 12 bits, 10 bits, 6 bits.
    let t = [4095, 4095, 1023, 1023, 4095, 4095, 63, 63];
    let d = Detailed::decode(&dtd(655_350, t, (4095, 4095), 0x18)).unwrap();
    assert_eq!([d.h_active, d.h_blank, d.h_front, d.h_sync], [4095, 4095, 1023, 1023]);
    assert_eq!([d.v_active, d.v_blank, d.v_front, d.v_sync], [4095, 4095, 63, 63]);
    assert_eq!((d.width_mm, d.height_mm), (4095, 4095));
    assert!(!d.h_positive && !d.v_positive, "polarity bits clear");
}

#[test]
fn an_interlaced_descriptor_counts_two_fields_to_a_frame() {
    let d = dtd(74_250, [1920, 280, 88, 44, 540, 22, 2, 5], (0, 0), 0x9E);
    let d = Detailed::decode(&d).unwrap();
    assert!(d.interlaced);
    assert_eq!((d.width(), d.height()), (1920, 1080));
}

#[test]
fn analog_sync_carries_no_polarity() {
    let d = dtd(25_175, [640, 160, 16, 96, 480, 45, 10, 2], (0, 0), 0x06);
    let d = Detailed::decode(&d).unwrap();
    assert!(!d.h_positive && !d.v_positive);
    let comp = dtd(25_175, [640, 160, 16, 96, 480, 45, 10, 2], (0, 0), 0x16);
    let comp = Detailed::decode(&comp).unwrap();
    assert!(comp.h_positive && !comp.v_positive, "digital composite: H only");
}

#[test]
fn a_zero_clock_or_an_empty_picture_is_not_a_timing() {
    assert_eq!(Detailed::decode(&[0u8; 18]), None);
    assert_eq!(Detailed::decode(&dtd(10, [0, 1, 1, 1, 1, 1, 1, 1], (0, 0), 0)), None);
    assert_eq!(Detailed::decode(&dtd_1080p()[..17]), None);
}
