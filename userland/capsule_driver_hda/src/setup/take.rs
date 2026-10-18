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

//! Taking a controller: its claim, its register window and its interrupt,
//! then a full reset of the controller and the link.

use super::mark::Line;
use super::sequence::claimed;
use super::{irq, mmio, pci};
use crate::constants::GCAP;
use crate::controller::{intel, reset, ControllerInfo};
use crate::discover::Found;
use crate::error::{HdaError, HdaResult};
use crate::handles::BrokerHandles;
use crate::regs::Regs;

/// Claim `dev`, bus mastering off, and map what the rest of the attempt
/// drives it through. A refusal before the handles exist gives the claim
/// back where it happens; from the handles on, every way out of the attempt
/// is their drop.
pub(super) fn take(dev: Found) -> HdaResult<(BrokerHandles, u64)> {
    let claim_epoch = claimed(&dev)?;
    pci::prepare(dev.device_id, claim_epoch, intel::pci_quirks(dev.vendor, dev.device));
    let mmio = mmio::map(dev.device_id, claim_epoch, dev.bar_size)?;
    let irq = irq::bind(dev, claim_epoch);
    let handles = BrokerHandles::new(dev.device_id, claim_epoch, &mmio, irq.grant_id);
    Ok((handles, claim_epoch))
}

/// Reset the controller and the link, and return what the controller is and
/// the codecs that answered. `window` is the length the broker mapped, which
/// an MSI-X table in the BAR can cut short of `dev.bar_size`.
pub(super) fn link_up(
    dev: Found,
    epoch: u64,
    regs: Regs,
    window: u64,
) -> HdaResult<(ControllerInfo, u16)> {
    let id = dev.device_id;
    Line::new("[HDA] controller ")
        .hex(dev.vendor as u32, 4)
        .s(":")
        .hex(dev.device as u32, 4)
        .emit();
    // SAFETY: GCAP lies at offset 0 of the mapped window.
    if unsafe { regs.r16(GCAP) } == 0xffff {
        return Err(HdaError::ControllerNotResponding);
    }
    let quirks = intel::pci_quirks(dev.vendor, dev.device);
    pci::clock_gating(id, epoch, quirks, false);
    let reset = reset::reset_link(regs);
    pci::clock_gating(id, epoch, quirks, true);
    let codec_mask = reset?;
    intel::reduce_dma_latency(regs, dev.vendor, dev.device, window);
    intel::init_link_clock(regs, dev.vendor, dev.device, window);
    let info = ControllerInfo::read(regs);
    if info.vmaj == 0 || info.gcap == 0 {
        return Err(HdaError::UnsupportedController);
    }
    Ok((info, codec_mask))
}
