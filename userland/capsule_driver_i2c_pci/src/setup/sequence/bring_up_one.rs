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
use nonos_libc::mk_device_release;

use super::unbound::unbound;
use super::unlisted_gate::unlisted_gate;
use crate::discover::Found;
use crate::driver::Driver;
use crate::init::{bring_up, BusSetup};
use crate::regs::Regs;
use crate::setup::{claim, mmio, pci};

/// Claim, map and bring up one controller. No interrupt is bound: the
/// engine polls, and a bound line would be authority the capsule never uses.
pub(super) fn bring_up_one(dev: Found, standard_mode: bool) -> Result<Driver, &'static str> {
    let claim_epoch = claim::claim(dev.device_id)?;
    if !dev.is_acpi {
        pci::enable(dev.device_id, claim_epoch).map_err(|e| release(dev.device_id, e))?;
    }
    let mmio = mmio::map(dev, claim_epoch)?;
    /*
     * SAFETY: the broker mapped `mmio.length` bytes at `mmio.user_va`, page
     * aligned, until the claim is released, which ends every use of `regs`.
     */
    let regs = unsafe { Regs::new(mmio.user_va, mmio.length) };
    unlisted_gate(&dev, regs).map_err(|e| release(dev.device_id, e))?;
    let lpss_base = dev.is_lpss().then_some(dev.bar0_base);
    let setup = BusSetup { clock_hz: dev.clock_hz, lpss_base, standard_mode };
    let init = bring_up(regs, setup).map_err(|e| release(dev.device_id, e))?;
    Ok(unbound(&dev, claim_epoch, mmio.grant_id, setup, init, regs))
}

/// Give the controller back when bringing it up fails: the kernel cascades
/// the teardown of its claim and BAR mapping, and `run.rs` can claim it again.
fn release(device_id: u64, why: &'static str) -> &'static str {
    let _ = mk_device_release(device_id);
    why
}
