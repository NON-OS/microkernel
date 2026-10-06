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

//! The bus voltage: first the host's highest, then the lowest voltage both
//! the host and the card's OCR name, as Linux chooses.

use crate::emmc::sdhci::caps::Caps;
use crate::emmc::sdhci::power::*;

fn host(v33: bool, v30: bool, v18: bool) -> u32 {
    let caps = (v33 as u32) << 24 | (v30 as u32) << 25 | (v18 as u32) << 26;
    host_ocr(&Caps { caps, caps1: 0, version: 2 })
}

#[test]
fn host_ocr_from_capabilities() {
    assert_eq!(host(true, false, false), 0x0030_0000);
    assert_eq!(host(false, true, false), 0x0006_0000);
    assert_eq!(host(false, false, true), 0x0000_0080);
    assert_eq!(host(true, true, true), 0x0036_0080);
    assert_eq!(host(false, false, false), 0);
}

#[test]
fn first_power_is_the_highest_the_host_offers() {
    assert_eq!(first_vdd(host(true, true, true)), Some(Vdd::V33));
    assert_eq!(first_vdd(host(false, true, true)), Some(Vdd::V30));
    assert_eq!(first_vdd(host(false, false, true)), Some(Vdd::V18));
    assert_eq!(first_vdd(0), None);
}

#[test]
fn an_emmc_settles_on_1v8_when_the_host_has_it() {
    assert_eq!(select(EMMC_DUAL_OCR, host(true, true, true)), Some((0x80, Vdd::V18)));
    assert_eq!(select(EMMC_DUAL_OCR, host(false, false, true)), Some((0x80, Vdd::V18)));
    assert_eq!(
        select(0x4000_0000 | EMMC_DUAL_OCR, host(false, false, true)),
        Some((0x80, Vdd::V18))
    );
}

#[test]
fn without_1v8_the_lowest_shared_window_wins() {
    assert_eq!(select(EMMC_DUAL_OCR, host(true, true, false)), Some((0x0006_0000, Vdd::V30)));
    assert_eq!(select(EMMC_DUAL_OCR, host(true, false, false)), Some((0x0030_0000, Vdd::V33)));
}

#[test]
fn no_shared_voltage_is_none() {
    // A 1.8 V-only card on a 3.3 V-only host.
    assert_eq!(select(0x0000_0080, host(true, false, false)), None);
    // A high voltage-only card on a 1.8 V-only host.
    assert_eq!(select(0x00ff_8000, host(false, false, true)), None);
    // Reserved low bits do not count.
    assert_eq!(select(0x7f, 0x7f), None);
}

#[test]
fn power_register_values() {
    assert_eq!(Vdd::V18.select(), 0x0a);
    assert_eq!(Vdd::V30.select(), 0x0c);
    assert_eq!(Vdd::V33.select(), 0x0e);
    assert_eq!(vdd_for_bit(7), Some(Vdd::V18));
    assert_eq!(vdd_for_bit(17), Some(Vdd::V30));
    assert_eq!(vdd_for_bit(20), Some(Vdd::V33));
    assert_eq!(vdd_for_bit(15), None);
}
