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

//! An internal PHY behind MDIC, answering as igc_read_phy_reg_mdic and
//! igc_write_phy_reg_mdic expect: a command without READY is carried out,
//! then READY is set with the data for a read. Only BMCR is modelled.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use nonos_devmodel::FakeBar;

use crate::constants::phy::{MDIC_DATA_MASK, MDIC_OP_READ, MDIC_OP_WRITE, MDIC_READY};
use crate::constants::regs::REG_MDIC;

use super::model::stable32;

/// BMCR as the firmware left it on a powered-down port: auto-negotiation
/// enabled, full duplex, 1000 Mb/s select bits, and POWER_DOWN set.
pub const BMCR_POWERED_DOWN: u32 = 0x1940;

pub fn phy(bmcr: Arc<AtomicU32>) -> impl Fn(&FakeBar) + Send + 'static {
    move |b| {
        let Some(cmd) = stable32(b, REG_MDIC) else { return };
        if cmd & MDIC_READY != 0 || cmd & (MDIC_OP_READ | MDIC_OP_WRITE) == 0 {
            return;
        }
        if cmd & MDIC_OP_WRITE != 0 {
            bmcr.store(cmd & MDIC_DATA_MASK, Ordering::SeqCst);
            b.present32(REG_MDIC, cmd | MDIC_READY);
        } else {
            let data = bmcr.load(Ordering::SeqCst);
            b.present32(REG_MDIC, (cmd & !MDIC_DATA_MASK) | MDIC_READY | data);
        }
    }
}

pub fn bmcr(start: u32) -> Arc<AtomicU32> {
    Arc::new(AtomicU32::new(start))
}
