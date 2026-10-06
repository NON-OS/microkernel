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

//! One PHY access by page and number, the way Linux addresses it: on the
//! PCH parts through __e1000_read_phy_reg_hv (page 0 at PHY address 2, pages
//! from 768 on at address 1, selected through register 0x1F), on the 82574
//! and 82583 through e1000e_read_phy_reg_bm2 at address 1. Only the IEEE
//! page 0 and pages 768 and up are used: the debug pages 1 to 767 and the
//! wake-up page 800 need other paths and nothing in the bring-up reads them.

use crate::constants::phy::{
    PhyReg, BM_PHY_ADDR, HV_INTC_FC_PAGE_START, IGP_PAGE_SHIFT, MAX_PHY_MULTI_PAGE_REG,
    PHY_PAGE_SELECT,
};
use crate::constants::Family;
use crate::regs::Regs;

use super::mdic::transact;

/// A read when `v` is None, else a write of `v`; a read answers the data.
pub fn once(
    regs: &Regs,
    family: Family,
    (page, reg): PhyReg,
    v: Option<u16>,
) -> Result<u16, &'static str> {
    if !family.is_pch() {
        return transact(regs, v.is_some(), BM_PHY_ADDR, reg, v.unwrap_or(0));
    }
    let high = page >= HV_INTC_FC_PAGE_START;
    if high && reg > MAX_PHY_MULTI_PAGE_REG {
        // e1000_set_page_igp: always at address 1; page 768 is selected as 0.
        let sel = if page == HV_INTC_FC_PAGE_START { 0 } else { page };
        transact(regs, true, 1, PHY_PAGE_SELECT, sel << IGP_PAGE_SHIFT)?;
    }
    transact(regs, v.is_some(), if high { 1 } else { 2 }, reg, v.unwrap_or(0))
}
