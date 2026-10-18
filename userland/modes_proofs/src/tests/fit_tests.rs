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

//! The display wire's fit rule.

use crate::fit::{fit, Range, CURSOR_SLOT};

fn range(w: u32, h: u32, align: u32, budget: u64) -> Range {
    Range { max_width: w, max_height: h, width_align: align, budget }
}

const BIG: u64 = 1 << 40;

#[test]
fn a_zero_size_or_the_current_size_asks_for_nothing() {
    let r = range(8192, 8192, 1, BIG);
    assert_eq!(fit((0, 900), (1024, 768), Some(&r), &[]), None);
    assert_eq!(fit((1600, 0), (1024, 768), Some(&r), &[]), None);
    assert_eq!(fit((1024, 768), (1024, 768), Some(&r), &[]), None);
}

#[test]
fn a_range_adapter_is_clamped_and_aligned() {
    let r = range(2560, 1600, 8, BIG);
    assert_eq!(fit((1601, 901), (1024, 768), Some(&r), &[]), Some((1600, 901)));
    assert_eq!(fit((5000, 5000), (1024, 768), Some(&r), &[]), Some((2560, 1600)));
    assert_eq!(fit((100, 100), (1024, 768), Some(&r), &[]), Some((640, 480)));
}

#[test]
fn a_range_adapter_shrinks_to_its_budget_keeping_the_shape() {
    let budget = 1920 * 1080 * 4 + CURSOR_SLOT;
    let r = range(8192, 8192, 8, budget);
    let (w, h) = fit((3840, 2160), (1024, 768), Some(&r), &[]).unwrap();
    assert!(w as u64 * h as u64 * 4 + CURSOR_SLOT <= budget && w % 8 == 0);
    assert!(w >= 1600 && (w as i64 * 9 - h as i64 * 16).abs() < 16 * 16, "{w}x{h}");
    assert_eq!(fit((800, 600), (1024, 768), Some(&range(8192, 8192, 1, 1000)), &[]), None);
}

#[test]
fn a_range_below_the_minimum_takes_nothing() {
    assert_eq!(fit((800, 600), (0, 0), Some(&range(320, 200, 1, BIG)), &[]), None);
}

#[test]
fn a_list_adapter_gets_its_largest_mode_inside_the_wanted_size() {
    let modes = [(1024, 768), (1920, 1080), (1600, 900), (1280, 1024), (2560, 1440)];
    assert_eq!(fit((1920, 1200), (1024, 768), None, &modes), Some((1920, 1080)));
    assert_eq!(fit((1700, 1000), (1024, 768), None, &modes), Some((1600, 900)));
    assert_eq!(fit((1300, 1024), (1024, 768), None, &modes), Some((1280, 1024)));
    assert_eq!(fit((1000, 700), (1024, 768), None, &modes), None, "nothing fits");
    assert_eq!(fit((1919, 1080), (1600, 900), None, &modes), None, "already there");
}
