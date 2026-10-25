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

//! A reset's own writes (L78): what it clears saved first, the marker set
//! and read back, and the reset itself, by the method chosen. All through
//! the kernel's config window; Command through the one lock every edit of it
//! takes (L73).

use super::bits_rule::{AF_INITIATE_FLR, D3HOT, INITIATE_FLR, INTX_DISABLE, MEM};
use super::caps::ResetCaps;
use super::marker_rule::{marker, Marker};
use super::method_rule::Method;
use super::start_rule::written;
use super::table::{Saved, Wipe};
use crate::drivers::pci::ConfigSpace;

const CLS: u16 = 0x0C;
const LT: u16 = 0x0D;
const INT_LINE: u16 = 0x3C;

pub(super) fn save(cfg: &ConfigSpace, c: ResetCaps, command: u16) -> Option<Saved> {
    let pcie = |off: u16| (c.pcie != 0).then(|| cfg.read16(u16::from(c.pcie) + off).ok()).flatten();
    Some(Saved {
        command,
        devctl: pcie(0x08),
        devctl2: if c.devctl2 { pcie(0x28) } else { None },
        cls: cfg.read8(CLS).ok()?,
        lt: cfg.read8(LT).ok()?,
        int_line: cfg.read8(INT_LINE).ok()?,
    })
}

/// INTx Disable set and read back, else Memory Space where a memory BAR
/// decodes; none when neither holds, and no reset is then trusted.
pub(super) fn mark(cfg: &ConfigSpace, c: ResetCaps) -> Option<Marker> {
    let sets = |bit: u16| {
        cfg.update_command(|v| v | bit).is_ok() && cfg.read16(0x04).is_ok_and(|v| v & bit != 0)
    };
    let intx = sets(INTX_DISABLE);
    marker(intx, !intx && c.mem_bar && sets(MEM))
}

pub(super) fn write(mut w: Wipe, m: Method, now: u64) -> Wipe {
    let (cfg, c) = (ConfigSpace::new(w.address), w.caps);
    let at = |cap: u8, off: u16| u16::from(cap) + off;
    let _ = match m {
        Method::Flr => {
            cfg.read16(at(c.pcie, 8)).and_then(|d| cfg.write16(at(c.pcie, 8), d | INITIATE_FLR))
        }
        Method::AfFlr => cfg.write8(at(c.af, 4), AF_INITIATE_FLR),
        Method::PmCycle => cfg
            .read16(at(c.pm, 4))
            .and_then(|p| cfg.write16(at(c.pm, 4), super::bits_rule::pmcsr_to(p, D3HOT))),
    };
    // Stamped once the write is made, so its quiet window starts there.
    w.method = Some(m);
    w.phase = written(m, crate::time::now_ns().max(now));
    w
}
