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

//! The rest of the LPSS wrapper bring-up Linux intel_lpss_init_dev does, the
//! component checks i2c-designware makes before trusting a core, and the
//! tables that tell an I2C function from its UART and SPI neighbours.

use crate::constants::{
    bxt_pad_offset, controller_index, device_info, is_bxt_family, IC_COMP_TYPE, IC_COMP_VERSION,
    IC_CON, IC_CON_SPEED_FAST, IC_CON_SPEED_STD, IC_SDA_HOLD, LPSS_PRIV_CAPS, LPSS_PRIV_REMAP_HI,
    LPSS_PRIV_REMAP_LO, LPSS_PRIV_RESETS,
};
use crate::init::{bring_up, BusSetup};
use crate::model::{live, refusal, CLOCK_HZ, PHYS_BASE, SETUP};
use crate::regs::Regs;

const SPEED_MASK: u32 = 3 << 1;

#[test]
fn the_remap_address_is_the_window_physical_base_both_halves() {
    let bar = live();
    bring_up(Regs::new(bar.base()), SETUP).expect("bring-up");
    assert_eq!(bar.wrote32(LPSS_PRIV_REMAP_LO as usize), PHYS_BASE as u32);
    assert_eq!(bar.wrote32(LPSS_PRIV_REMAP_HI as usize), (PHYS_BASE >> 32) as u32);
}

#[test]
fn an_lpss_uart_or_spi_function_is_refused_by_its_capabilities() {
    let bar = live();
    bar.present32(LPSS_PRIV_CAPS as usize, 1 << 4);
    let err = refusal(bring_up(Regs::new(bar.base()), SETUP));
    assert!(err.contains("not I2C"), "unexpected error: {err}");
    let bar = live();
    bar.present32(LPSS_PRIV_CAPS as usize, 2 << 4);
    assert!(bring_up(Regs::new(bar.base()), SETUP).is_err());
}

#[test]
fn a_core_that_is_not_designware_i2c_is_refused() {
    let bar = live();
    bar.present32(IC_COMP_TYPE as usize, 0x4457_0110);
    let err = refusal(bring_up(Regs::new(bar.base()), SETUP));
    assert!(err.contains("not a DesignWare I2C core"), "unexpected error: {err}");
}

#[test]
fn a_platform_controller_has_no_lpss_block_and_none_is_written() {
    // AMD's AMDI0010 is a bare DesignWare core: 0x200 and up is not LPSS.
    let bar = live();
    let setup = BusSetup { clock_hz: 150_000_000, lpss_base: None, standard_mode: false };
    bring_up(Regs::new(bar.base()), setup).expect("bring-up");
    for reg in [LPSS_PRIV_RESETS, LPSS_PRIV_REMAP_LO, LPSS_PRIV_REMAP_HI] {
        assert_eq!(bar.wrote32(reg as usize), 0, "LPSS register {reg:#x} written");
    }
}

#[test]
fn a_device_that_declares_100_khz_gets_standard_mode() {
    let bar = live();
    let setup = BusSetup { standard_mode: true, ..SETUP };
    bring_up(Regs::new(bar.base()), setup).expect("bring-up");
    assert_eq!(bar.wrote32(IC_CON as usize) & SPEED_MASK, IC_CON_SPEED_STD);
    let bar = live();
    bring_up(Regs::new(bar.base()), SETUP).expect("bring-up");
    assert_eq!(bar.wrote32(IC_CON as usize) & SPEED_MASK, IC_CON_SPEED_FAST);
}

#[test]
fn a_core_older_than_1_11a_has_no_sda_hold_register_and_it_is_left_alone() {
    let bar = live();
    bar.present32(IC_COMP_VERSION as usize, 0x3130_302A);
    bring_up(Regs::new(bar.base()), SETUP).expect("bring-up");
    assert_eq!(bar.wrote32(IC_SDA_HOLD as usize), 0);
}

#[test]
fn only_i2c_functions_are_in_the_table_with_their_linux_clocks() {
    // Gemini Lake, the HP 15s-fq0xxx's SoC: I2C0..I2C7 at 133 MHz.
    for id in (0x31ACu16..=0x31BA).step_by(2) {
        assert_eq!(device_info(id).map(|i| i.1), Some(CLOCK_HZ), "{id:#06x}");
        assert!(is_bxt_family(id));
    }
    // UART and SPI functions of the same LPSS blocks (Gemini Lake, Apollo
    // Lake, Sunrise Point-LP, Tiger Lake-LP) must never be taken for I2C.
    for id in [0x31BC, 0x31C2, 0x5ABC, 0x5AC2, 0x5AC4, 0x5AC6, 0x5AEE, 0x9D27, 0x9D29, 0xA0A8, 0xA0DA] {
        assert!(device_info(id).is_none(), "{id:#06x} is not an I2C function");
    }
    assert_eq!(device_info(0x5AAC).map(|i| i.1), Some(133_000_000));
    assert_eq!(device_info(0x9D60).map(|i| i.1), Some(120_000_000));
    assert_eq!(device_info(0x9DE8).map(|i| i.1), Some(216_000_000));
    assert_eq!(device_info(0x02E9).map(|i| i.1), Some(216_000_000));
    assert!(!is_bxt_family(0x9D60) && !is_bxt_family(0xA0E8));
}

#[test]
fn firmware_i2c_names_map_to_pci_functions_on_every_listed_platform() {
    assert_eq!(controller_index(0x31AC), Some(0));
    assert_eq!(controller_index(0x31B4), Some(4));
    assert_eq!(controller_index(0x31BA), Some(7));
    assert_eq!(controller_index(0x31BC), None);
    assert_eq!(controller_index(0x9D61), Some(1));
    assert_eq!(controller_index(0x9DC6), Some(5));
    assert_eq!(controller_index(0xA0D9), Some(7));
    assert_eq!(controller_index(0x51C5), Some(4));
    assert_eq!(controller_index(0x7E7B), Some(3));
}

#[test]
fn a_gemini_lake_pad_lies_inside_its_community_window() {
    // PADBAR 0x500, pad 18: 0x500 + 18 * 8.
    assert_eq!(bxt_pad_offset(0x500, 18, 0x4000), Some(0x590));
    assert_eq!(bxt_pad_offset(0x500, 2000, 0x4000), None, "pad past the window");
    assert_eq!(bxt_pad_offset(0, 18, 0x4000), None, "dead PADBAR");
    assert_eq!(bxt_pad_offset(u32::MAX, 18, 0x4000), None, "undecoded window");
}
