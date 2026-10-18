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
//! The Intel and AMD settings written to PCI configuration space around the
//! controller reset, each through `config::update`.

use super::config::update;
use crate::constants::{
    CGCTL_MISCBDCGE, DEVC_NOSNOOP, PCI_CFG_CGCTL, PCI_CFG_DEVC, PCI_CFG_TCSEL, TCSEL_MASK,
};
use crate::controller::intel::{PciQuirks, AMD_ENABLE_SNOOP, AMD_MISC_CNTR2, AMD_SNOOP_MASK};
use crate::error::{HdaError, HdaResult};
use crate::setup::Line;

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
