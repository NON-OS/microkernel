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

//! The rest of the LPSS wrapper bring-up, and the component checks made
//! before a core is trusted.

use crate::constants::{
    IC_COMP_TYPE, LPSS_PRIV_CAPS, LPSS_PRIV_REMAP_HI, LPSS_PRIV_REMAP_LO, LPSS_PRIV_RESETS,
};
use crate::init::{bring_up, BusSetup};
use crate::model::live;
use crate::setup::{refusal, regs, PHYS_BASE, SETUP};

#[test]
fn the_remap_address_is_the_window_physical_base_both_halves() {
    let bar = live();
    bring_up(regs(&bar), SETUP).expect("bring-up");
    assert_eq!(bar.wrote32(LPSS_PRIV_REMAP_LO as usize), PHYS_BASE as u32);
    assert_eq!(bar.wrote32(LPSS_PRIV_REMAP_HI as usize), (PHYS_BASE >> 32) as u32);
}

#[test]
fn an_lpss_uart_or_spi_function_is_refused_by_its_capabilities() {
    let bar = live();
    bar.present32(LPSS_PRIV_CAPS as usize, 1 << 4);
    let err = refusal(bring_up(regs(&bar), SETUP));
    assert!(err.contains("not I2C"), "unexpected error: {err}");
    let bar = live();
    bar.present32(LPSS_PRIV_CAPS as usize, 2 << 4);
    assert!(bring_up(regs(&bar), SETUP).is_err());
}

#[test]
fn a_core_that_is_not_designware_i2c_is_refused() {
    let bar = live();
    bar.present32(IC_COMP_TYPE as usize, 0x4457_0110);
    let err = refusal(bring_up(regs(&bar), SETUP));
    assert!(err.contains("not a DesignWare I2C core"), "unexpected error: {err}");
}

#[test]
fn a_platform_controller_has_no_lpss_block_and_none_is_written() {
    // AMD's AMDI0010 is a bare DesignWare core: 0x200 and up is not LPSS.
    let bar = live();
    let setup = BusSetup { clock_hz: 150_000_000, lpss_base: None, standard_mode: false };
    bring_up(regs(&bar), setup).expect("bring-up");
    for reg in [LPSS_PRIV_RESETS, LPSS_PRIV_REMAP_LO, LPSS_PRIV_REMAP_HI] {
        assert_eq!(bar.wrote32(reg as usize), 0, "LPSS register {reg:#x} written");
    }
}
