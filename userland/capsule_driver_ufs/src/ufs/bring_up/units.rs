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

//! The last stages: the units REPORT LUNS names, each opened and served or
//! named as skipped. A device with none to serve is a failed bring-up.

use super::say::{failed, unit, unit_skipped};
use super::stage::Stage;
use crate::ufs::ctx::Ctx;
use crate::ufs::disk::{unit::open, Ufs};
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::scsi::luns::{self, MAX_UNITS};

pub fn serve<H: Hw>(hw: &H, ctx: Ctx) -> Result<Ufs, UfsError> {
    let luns = luns::report(hw, &ctx).map_err(|e| failed(hw, Stage::Luns.name(), e))?;
    let (found, n) = luns;
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
        return Err(failed(hw, Stage::Units.name(), UfsError::NoDevice));
    }
    Ok(Ufs { ctx, units })
}
