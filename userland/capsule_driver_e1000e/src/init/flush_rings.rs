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

//! Linux e1000_flush_desc_rings, run before the reset on pch_spt and later:
//! an I219 reset while it still holds descriptors enters a unit hang only a
//! PCI reset clears. Config word PCICFG_DESC_RING_STATUS says whether it
//! does; the broker lets a claimed device's config space be read (any
//! aligned offset below 256), so the check is the real one.

use nonos_libc::mk_pci_config_read;

use crate::constants::pch_bits::{
    FEXTNVM11_DISABLE_MULR_FIX, FLUSH_DESC_REQUIRED, PCICFG_DESC_RING_STATUS,
};
use crate::constants::regs::REG_TDLEN;
use crate::constants::regs_pch::REG_FEXTNVM11;
use crate::log::say;
use crate::setup::Driver;

use super::{flush_rx, flush_tx};

pub fn run(d: &Driver) {
    // SAFETY: `d.regs` is the broker-mapped BAR0 window; FEXTNVM11 and TDLEN
    // are 4-byte registers inside it on pch_spt and later.
    let tdlen = unsafe {
        d.regs.modify(REG_FEXTNVM11, 0, FEXTNVM11_DISABLE_MULR_FIX);
        d.regs.r32(REG_TDLEN)
    };
    let Some(hang) = ring_status(d) else {
        say("DESC_RING_STATUS config read refused, ring flush skipped");
        return;
    };
    if hang & FLUSH_DESC_REQUIRED == 0 || tdlen == 0 {
        return;
    }
    say("descriptor rings left pending by firmware, flushing");
    flush_tx::run(d);
    // The transmit flush may have been enough; the receive ring is flushed
    // only when the part still asks for it.
    if ring_status(d).is_some_and(|h| h & FLUSH_DESC_REQUIRED != 0) {
        flush_rx::run(&d.regs);
    }
}

fn ring_status(d: &Driver) -> Option<u16> {
    let r = mk_pci_config_read(d.device_id, d.claim_epoch, PCICFG_DESC_RING_STATUS, 2);
    if r < 0 {
        None
    } else {
        Some(r as u16)
    }
}
