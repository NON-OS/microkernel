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
use nonos_libc::{
    mk_pci_config_read, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_MEMORY_SPACE,
};

use crate::constants::{
    CGCTL_MISCBDCGE, DEVC_NOSNOOP, PCI_CFG_CGCTL, PCI_CFG_DEVC, PCI_CFG_TCSEL, TCSEL_MASK,
};
use crate::controller::intel::{with_bits, PciQuirks, AMD_ENABLE_SNOOP, AMD_MISC_CNTR2, AMD_SNOOP_MASK};
use crate::error::{HdaError, HdaResult};
use crate::setup::Line;

/// Bus mastering for the rings and the stream, and memory decoding for the
/// register BAR, which firmware leaves off on a controller it did not use.
pub fn enable(device_id: u64, claim_epoch: u64) -> HdaResult<()> {
    let bits = MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_MEMORY_SPACE;
    let r = mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, bits);
    if r < 0 {
        return Err(HdaError::BrokerCallFailed(r));
    }
    Ok(())
}

/// One PCI configuration word read, its `mask` bits set to `want`, and
/// written back when they differ. The broker refuses any other bit.
fn update(device_id: u64, claim_epoch: u64, offset: u32, mask: u16, want: u16) -> HdaResult<()> {
    let cur = mk_pci_config_read(device_id, claim_epoch, offset, 2);
    if cur < 0 {
        return Err(HdaError::BrokerCallFailed(cur));
    }
    let cur = cur as u16;
    let new = with_bits(cur, mask, want);
    if new == cur {
        return Ok(());
    }
    let r = mk_pci_config_write(device_id, claim_epoch, offset, new);
    if r < 0 {
        return Err(HdaError::BrokerCallFailed(r));
    }
    Ok(())
}

/// The settings `azx_init_pci` writes before the controller is touched:
/// TCSEL to 0 on Intel, snooping on for the Intel PCH (DEVC NOSNOOP off)
/// and on AMD (MISC_CNTR2 ENABLE_SNOOP). A write the broker refuses is
/// logged and passed over; the DMA buffers are flushed from the cache
/// either way.
pub fn prepare(device_id: u64, claim_epoch: u64, q: PciQuirks) {
    if q.clear_tcsel {
        say(update(device_id, claim_epoch, PCI_CFG_TCSEL, TCSEL_MASK as u16, 0), "tcsel");
    }
    if q.clear_nosnoop {
        say(update(device_id, claim_epoch, PCI_CFG_DEVC, DEVC_NOSNOOP as u16, 0), "nosnoop");
    }
    if q.amd_snoop {
        let r = update(device_id, claim_epoch, AMD_MISC_CNTR2, AMD_SNOOP_MASK, AMD_ENABLE_SNOOP);
        say(r, "amd snoop");
    }
}

/// CGCTL MISCBDCGE off (`gated` false) before the controller reset and on
/// again after it, as `hda_intel_init_chip` does on Skylake and later.
pub fn clock_gating(device_id: u64, claim_epoch: u64, q: PciQuirks, gated: bool) {
    if !q.gate_cgctl {
        return;
    }
    let bit = CGCTL_MISCBDCGE as u16;
    let r = update(device_id, claim_epoch, PCI_CFG_CGCTL, bit, if gated { bit } else { 0 });
    say(r, if gated { "cgctl on" } else { "cgctl off" });
}

fn say(r: HdaResult<()>, what: &str) {
    if let Err(HdaError::BrokerCallFailed(code)) = r {
        Line::new("[HDA] pci ").s(what).s(" not written, broker said ").dec(code as u32).emit();
    }
}
