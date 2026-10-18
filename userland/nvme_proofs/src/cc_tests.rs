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

//! The CC written at enable, against CAP values a controller may report.

use crate::admin::cc_value;
use crate::error::NvmeError;

const CSS_NVM: u64 = 1 << 37;
const CSS_IO_SETS: u64 = 1 << 43;
const CSS_ADMIN_ONLY: u64 = 1 << 44;

fn cap(mpsmin: u64, mpsmax: u64, css: u64) -> u64 {
    (mpsmin << 48) | (mpsmax << 52) | css | 0x0f00_03ff
}

#[test]
fn a_4k_controller_gets_mps_0_the_nvm_set_and_the_entry_sizes() {
    let cc = cc_value(cap(0, 4, CSS_NVM)).expect("served");
    assert_eq!(cc & 1, 1, "EN");
    assert_eq!((cc >> 4) & 0x7, 0, "CSS: the NVM command set");
    assert_eq!((cc >> 7) & 0xf, 0, "MPS: MPSMIN, 4 KiB");
    assert_eq!((cc >> 11) & 0x7, 0, "AMS: round robin");
    assert_eq!((cc >> 14) & 0x3, 0, "no shutdown asked");
    assert_eq!((cc >> 16) & 0xf, 6, "IOSQES: 64 bytes");
    assert_eq!((cc >> 20) & 0xf, 4, "IOCQES: 16 bytes");
    assert_eq!(cc >> 24, 0, "reserved and CRIME left clear");
}

#[test]
fn a_controller_whose_smallest_page_is_over_4k_is_refused() {
    for mpsmin in 1..=15 {
        assert_eq!(cc_value(cap(mpsmin, 15, CSS_NVM)), Err(NvmeError::UnsupportedPageSize));
    }
}

#[test]
fn a_controller_without_the_nvm_command_set_is_refused() {
    for css in [0, CSS_IO_SETS, CSS_ADMIN_ONLY, CSS_IO_SETS | CSS_ADMIN_ONLY] {
        assert_eq!(cc_value(cap(0, 4, css)), Err(NvmeError::NoNvmCommandSet));
    }
    assert!(cc_value(cap(0, 4, CSS_NVM | CSS_IO_SETS)).is_ok(), "NVM among others is enough");
}

#[test]
fn a_page_range_upside_down_is_refused() {
    assert_eq!(cc_value(cap(2, 1, CSS_NVM)), Err(NvmeError::UnsupportedController));
}

#[test]
fn no_cap_value_yields_a_cc_with_bits_outside_the_fields_written() {
    let mut x: u64 = 0x9e37_79b9_7f4a_7c15;
    for _ in 0..100_000 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        if let Ok(cc) = cc_value(x) {
            assert_eq!(cc, 0x0046_0001, "only one CC is ever written: {x:#x}");
        }
    }
}
