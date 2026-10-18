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

//! What the tests bring a window up with: an Intel LPSS function's clock and
//! physical base, the driver's own accessor over the window, and the
//! message a refused bring-up ends in.

use nonos_devmodel::FakeBar;

use crate::init::{BusSetup, InitState};
use crate::regs::Regs;

/// Gemini Lake's I2C input clock, the part this capsule was brought up on.
pub const CLOCK_HZ: u32 = 133_000_000;
/// Where the window sits physically, as the LPSS remap register must learn.
/// Above 4 GiB on purpose: firmware places LPSS BARs there on large-memory
/// machines, and a remap that dropped the high dword would point the
/// integrated DMA at the wrong page.
pub const PHYS_BASE: u64 = 0x0000_0040_1234_5000;
/// The bring-up an Intel LPSS function gets: its clock and its window.
pub const SETUP: BusSetup =
    BusSetup { clock_hz: CLOCK_HZ, lpss_base: Some(PHYS_BASE), standard_mode: false };

/// The driver's register accessor over `bar`.
pub fn regs(bar: &FakeBar) -> Regs {
    /*
     * SAFETY: the window is `bar.len()` bytes of host memory from the
     * allocator, which aligns it past a dword, and it outlives the test that
     * holds `bar` and uses the accessor.
     */
    unsafe { Regs::new(bar.base(), bar.len() as u64) }
}

/// The message a bring-up gave up with. `InitState` carries no `Debug`, so
/// `expect_err` is not available and the success case is spelled out here.
pub fn refusal(outcome: Result<InitState, &'static str>) -> &'static str {
    match outcome {
        Ok(_) => panic!("bring-up succeeded where this window should have refused it"),
        Err(message) => message,
    }
}
