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
//! A whole part for the bring-up tests: the global reset completes, and the
//! PHY answers behind MDIC.

use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use nonos_devmodel::FakeBar;

use crate::facts::mac::*;
use crate::facts::regs::*;

use super::model::stable32;
use super::phy_model::paged;

/// A part that finishes a global reset on the next look: CTRL.RST and the
/// master disable drop, and EECD reports the NVM fitted and read. `page` is
/// the I210/I211 PHY's page register, None for an unpaged PHY.
pub fn part(held: Arc<AtomicU32>, page: Option<Arc<AtomicU32>>) -> impl Fn(&FakeBar) + Send {
    let mdio = paged(held, page);
    move |b| {
        let ctrl = stable32(b, REG_CTRL).unwrap_or(0);
        if ctrl & CTRL_RST != 0 {
            b.present32(REG_CTRL, ctrl & !(CTRL_RST | CTRL_GIO_MASTER_DISABLE));
            b.present32(REG_EECD, EECD_AUTO_RD | EECD_PRES);
        }
        mdio(b);
    }
}
