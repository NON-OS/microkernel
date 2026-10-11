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

//! The bring-up in Linux's order (ufshcd_hba_enable, ufshcd_link_startup,
//! ufshcd_make_hba_operational, ufshcd_verify_dev_init,
//! ufshcd_complete_dev_init), then each logical unit REPORT LUNS names.
//! The link stays in PWM-G1, one lane, as link startup leaves it: Linux
//! then moves to a high-speed gear, which this driver does not yet do.

use super::say::{failed, unit, unit_skipped};
use crate::ufs::ctx::Ctx;
use crate::ufs::dev::{init, nop};
use crate::ufs::disk::{unit::open, Ufs};
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::hci::{caps, enable, link, lists};
use crate::ufs::scsi::luns::{self, MAX_UNITS};
use crate::ufs::utp::layout::Lists;

pub fn bring_up<H: Hw>(hw: &H, l: Lists, intel: bool) -> Result<Ufs, UfsError> {
    enable::restart(hw).map_err(|e| failed(hw, b"host enable", e))?;
    let caps = caps::read(hw);
    let regions = [l.utrl, l.utmrl, l.ucd, l.data];
    if !caps.a64 && regions.iter().any(|r| r.bus + r.len as u64 > 1 << 32) {
        return Err(failed(hw, b"DMA", UfsError::Dma32));
    }
    link::startup(hw, intel).map_err(|e| failed(hw, b"link startup", e))?;
    lists::run(hw, l.utrl.bus, l.utmrl.bus).map_err(|e| failed(hw, b"transfer lists", e))?;
    let ctx = Ctx { lists: l, caps, wedged: core::cell::Cell::new(false) };
    nop::verify(hw, &ctx).map_err(|e| failed(hw, b"NOP OUT", e))?;
    init::complete(hw, &ctx).map_err(|e| failed(hw, b"fDeviceInit", e))?;
    let (found, n) = luns::report(hw, &ctx).map_err(|e| failed(hw, b"REPORT LUNS", e))?;
    let mut units = [None; MAX_UNITS];
    let mut served = 0;
    for &lun in &found[..n] {
        match open(hw, &ctx, lun) {
            Ok(u) => {
                unit(hw, &u);
                units[served] = Some(u);
                served += 1;
            }
            Err(e) => unit_skipped(hw, lun, e),
        }
    }
    if served == 0 {
        return Err(failed(hw, b"logical units", UfsError::NoDevice));
    }
    Ok(Ufs { ctx, units })
}
