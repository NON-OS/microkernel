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

//! The windows the tests put in front of the driver.
//!
//! A passive window reads back what was written, which is what the
//! DesignWare core does for its configuration registers. What it does not do
//! is move IC_ENABLE_STATUS when IC_ENABLE is written, so a test that wants
//! the driver to see the controller as stuck enabled, or as already
//! quiescent, presents that status itself and the driver's wait loops behave
//! exactly as they would against a part in that state.

use nonos_devmodel::FakeBar;

use crate::constants::{IC_COMP_TYPE, IC_COMP_VERSION, IC_ENABLE, IC_ENABLE_STATUS};

/// Wide enough for the LPSS private block at 0x200, past the core's 0x100.
pub const WINDOW: usize = 0x400;
/// IC_COMP_VERSION of a 2.01a core, which has IC_SDA_HOLD.
pub const DW_COMP_VERSION: u32 = 0x3230_312A;
/// The DesignWare I2C component signature a live core answers with.
pub const DW_COMP_TYPE: u32 = 0x4457_0140;

/// A window every byte of which reads back `byte`, as an undecoded or
/// unpowered BAR does.
pub fn stuck_at(byte: u8) -> FakeBar {
    let bar = FakeBar::new(WINDOW);
    for offset in 0..WINDOW {
        bar.present8(offset, byte);
    }
    bar
}

/// A powered core that answers with its signature and reports itself already
/// disabled, so the driver's disable wait clears on the first read.
pub fn live() -> FakeBar {
    let bar = FakeBar::new(WINDOW);
    bar.present32(IC_COMP_TYPE as usize, DW_COMP_TYPE);
    bar.present32(IC_COMP_VERSION as usize, DW_COMP_VERSION);
    bar
}

/// A powered core that never lets go of its enable bit. IC_ENABLE_STATUS
/// stays set whatever the driver writes to IC_ENABLE, which is how a part
/// mid-transfer, or one whose functional clock is gated, behaves.
pub fn stuck_enabled() -> FakeBar {
    let bar = live();
    bar.present32(IC_ENABLE as usize, 1);
    bar.present32(IC_ENABLE_STATUS as usize, 1);
    bar
}

/// Enable bit set, status already quiescent, so the disable takes on the
/// first read. A restore then waits on a status this passive window never
/// raises, so it is observed in IC_ENABLE, not in a return value.
pub fn enabled_and_quiescent() -> FakeBar {
    let bar = live();
    bar.present32(IC_ENABLE as usize, 1);
    bar
}
