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

//! Claim the wifi device, enable its memory space and map its register window
//! through the broker. The claim epoch and device id are carried out so the
//! later firmware download can map DMA memory against the same claim.

use nonos_libc::{mk_device_claim, mk_device_release, mk_mmio_map, MmioMapOut};

use core::sync::atomic::Ordering;

use super::mapped::Mapped;
use super::window::{BAR_INDEX, WINDOW_VA};
use crate::discover::Found;
use crate::regs::Regs;

/// Claim and map the chip discovery found. Returns the register window ready
/// for power-on, or an error string for the on-screen status. A failure after
/// the claim gives the claim back (and with it anything mapped), so the next
/// attempt can claim the chip again; it used to keep it.
pub fn run(dev: Found) -> Result<Mapped, &'static str> {
    let epoch = mk_device_claim(dev.device_id);
    if epoch <= 0 {
        return Err("rtl8821ce: claim failed");
    }
    // Memory Space alone, so the MMIO BAR decodes. Bus Master waits until the
    // power cycle shows the MAC was off (`pwr::power_on_then_master`): a card
    // whose last driver died without its Drop may still have its DMA engines
    // armed at memory the kernel freed.
    if !super::master::command(dev.device_id, epoch as u64, false) {
        let _ = mk_device_release(dev.device_id);
        return Err("rtl8821ce: memory space enable failed");
    }
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let len = core::cmp::max(dev.bar_size, 0x1000);
    let r = mk_mmio_map(dev.device_id, epoch as u64, dev.bar_index, 0, 0, len, &mut out);
    if r < 0 {
        let _ = mk_device_release(dev.device_id);
        return Err("rtl8821ce: mmio map failed");
    }
    BAR_INDEX.store(dev.bar_index, Ordering::Relaxed);
    WINDOW_VA.store(out.user_va as u32, Ordering::Relaxed);
    Ok(Mapped {
        regs: Regs::new(out.user_va),
        device_id: dev.device_id,
        claim_epoch: epoch as u64,
        efuse: None,
    })
}
