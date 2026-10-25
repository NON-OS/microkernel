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

//! A claimed and mapped controller's core, brought up. An LPSS function's
//! remap is programmed with its window's physical base, which the device
//! list shows only to the device's holder: discovery, before the claim,
//! read it as 0, so it is read back from the controller's own record now.

use super::unlisted_gate::unlisted_gate;
use crate::discover::Found;
use crate::init::{bring_up, BusSetup, InitState};
use crate::regs::Regs;

/// Bring up `dev`'s core through `regs`. An LPSS function's base comes from
/// `reread`, given its device_id: None when the record cannot be read back.
/// A base it cannot read, or one that still reads 0, refuses the function
/// before any register is touched, so the remap never names address 0. A
/// platform controller has no remap and is not read back.
pub fn brought_up(
    dev: &Found,
    reread: impl FnOnce(u64) -> Option<u64>,
    regs: Regs,
    standard_mode: bool,
) -> Result<(BusSetup, InitState), &'static str> {
    let lpss_base = if dev.is_lpss() {
        let base = reread(dev.device_id).filter(|&base| base != 0);
        Some(base.ok_or("i2c-pci: LPSS window base unreadable after the claim, not remapped")?)
    } else {
        None
    };
    unlisted_gate(dev, regs)?;
    let setup = BusSetup { clock_hz: dev.clock_hz, lpss_base, standard_mode };
    Ok((setup, bring_up(regs, setup)?))
}
