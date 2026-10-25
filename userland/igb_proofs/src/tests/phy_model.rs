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
//! An internal PHY behind MDIC: a command without READY is carried out,
//! then READY is set with the data for a read. Like QEMU's igb, only PHY
//! address 1 answers; any other ends in ERROR. BMCR is modelled, and on a
//! paged PHY (the I210/I211) only on page 0: elsewhere register 0 is
//! another register, which reads 0xFFFF and takes no write.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use nonos_devmodel::FakeBar;

use crate::facts::phy::*;
use crate::facts::regs::REG_MDIC;

use super::model::stable32;

/// BMCR as the firmware left it on a powered-down port: auto-negotiation
/// enabled, full duplex, 1000 Mb/s select bits, and POWER_DOWN set.
pub const BMCR_POWERED_DOWN: u32 = 0x1940;

pub fn phy(bmcr: Arc<AtomicU32>) -> impl Fn(&FakeBar) + Send + 'static {
    paged(bmcr, None)
}

/// A paged PHY whose page register starts at `page`.
pub fn paged(bmcr: Arc<AtomicU32>, page: Option<Arc<AtomicU32>>) -> impl Fn(&FakeBar) + Send {
    move |b| {
        let Some(cmd) = stable32(b, REG_MDIC) else { return };
        if cmd & MDIC_READY != 0 || cmd & (MDIC_OP_READ | MDIC_OP_WRITE) == 0 {
            return;
        }
        // 1 written out, not PHY_ADDR: the model is QEMU's, not the driver's.
        if (cmd >> MDIC_PHY_SHIFT) & 0x1F != 1 {
            b.present32(REG_MDIC, cmd | MDIC_READY | MDIC_ERROR);
            return;
        }
        let reg = (cmd >> MDIC_REG_SHIFT) & 0x1F;
        let on_page_0 = page.as_ref().is_none_or(|p| p.load(Ordering::SeqCst) == 0);
        let data = cmd & MDIC_DATA_MASK;
        let write = cmd & MDIC_OP_WRITE != 0;
        match (reg, &page) {
            (GS40G_PAGE_SELECT, Some(p)) if write => p.store(data, Ordering::SeqCst),
            (PHY_CONTROL, _) if write && on_page_0 => bmcr.store(data, Ordering::SeqCst),
            _ => {}
        }
        let value = match reg {
            PHY_CONTROL if on_page_0 => bmcr.load(Ordering::SeqCst),
            _ => 0xFFFF,
        };
        let back = if write { cmd } else { (cmd & !MDIC_DATA_MASK) | value };
        b.present32(REG_MDIC, back | MDIC_READY);
    }
}

pub fn bmcr(start: u32) -> Arc<AtomicU32> {
    Arc::new(AtomicU32::new(start))
}
