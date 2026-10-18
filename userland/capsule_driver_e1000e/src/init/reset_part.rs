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

//! The part stopped and reset, in Linux's order (e1000_probe, e1000e_reset,
//! init_hw): quiesce, the PCH PHY made reachable, the global reset, the
//! post-reset bits, the family's hardware bits, K1 on pch_mtp and later.
//! The I219 flush in the quiesce may have turned Bus Master on, on this
//! driver's own rings; it goes off again before the reset, which then lands
//! with the part unable to reach memory. The reset leaves both units off; a
//! part that shows either on after it is named, and never given Bus Master.
//! An I219 holding a previous owner's descriptors is not flushed, and a
//! reset that does not complete on it is named for what clears it.

use crate::constants::regs::{REG_RCTL, REG_TCTL};
use crate::constants::rxtx::{RCTL_EN, TCTL_EN};
use crate::constants::Family;
use crate::setup::{command, Driver};
use crate::swflag;

use super::run::warn;
use super::{hw_bits_82574, hw_bits_pch, k1, pch_prepare, post_reset, quiesce, reset};

/// An I219 whose reset did not complete over a previous owner's descriptors:
/// they may name freed frames, and only a PCI function reset clears them.
pub const DESCRIPTORS_PENDING: &str = "I219 descriptors pending, a function reset is needed";

pub fn run(d: &Driver) -> Result<(), &'static str> {
    let family = d.family;
    let theirs = quiesce::run(d);
    command(d.device_id, d.claim_epoch, false)?;
    if family.is_pch() {
        pch_prepare::run(d)?;
    }
    reset::run(&d.regs, family).map_err(|e| if theirs { DESCRIPTORS_PENDING } else { e })?;
    // SAFETY: `d.regs` is the broker-mapped BAR0 window; RCTL and TCTL are
    // 4-byte registers inside it on every family.
    let on = unsafe { (d.regs.r32(REG_RCTL) & RCTL_EN) | (d.regs.r32(REG_TCTL) & TCTL_EN) };
    if on != 0 {
        return Err("RCTL/TCTL still on after reset");
    }
    post_reset::run(&d.regs, family);
    if family.is_pch() {
        hw_bits_pch::run(&d.regs, family);
    } else {
        hw_bits_82574::run(&d.regs);
    }
    if family >= Family::PchMtp {
        let k1 = swflag::acquire(&d.regs, family).and_then(|()| {
            let r = k1::reconfigure(&d.regs, family);
            swflag::release(&d.regs);
            r
        });
        // e1000_init_hw_ich8lan returns this error, and e1000e_reset only
        // logs it: the part still moves frames without the new K1 timing.
        warn(k1.err());
    }
    Ok(())
}
