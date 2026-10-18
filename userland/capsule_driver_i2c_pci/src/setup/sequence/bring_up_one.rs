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

use super::brought_up::brought_up;
use super::unbound::unbound;
use crate::discover::{claimed_base, Found};
use crate::driver::Driver;
use crate::regs::Regs;
use crate::setup::{claim, mmio, pci};

/// Claim, map and bring up one controller, its window's base read back from
/// its record now that it is held (brought_up.rs). No interrupt is bound:
/// the engine polls, and a bound line would be authority the capsule never
/// uses.
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
    let (setup, init) = brought_up(&dev, claimed_base, regs, standard_mode)
        .map_err(|e| release(dev.device_id, e))?;
    Ok(unbound(&dev, claim_epoch, mmio.grant_id, setup, init, regs))
}

/// Give the controller back when bringing it up fails: the kernel cascades
/// the teardown of its claim and BAR mapping, and `run.rs` can claim it again.
fn release(device_id: u64, why: &'static str) -> &'static str {
    let _ = mk_device_release(device_id);
    why
}
