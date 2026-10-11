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

//! ufshcd_link_startup: DME_LINKSTARTUP, retried three times with the host
//! reset between tries (a failed startup can leave the local UniPro stack
//! wanting one), then a device must be present on the link. Intel's hosts
//! have Line Control Commands cleared before each try.

use super::regs::{HCS, HCS_DP, UECPA};
use super::uic_codes::{DME_LINKSTARTUP, PA_LOCAL_TX_LCC_ENABLE};
use super::{dme, enable, uic};
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;

/// DME_LINKSTARTUP_RETRIES.
const RETRIES: usize = 3;

pub fn startup<H: Hw>(hw: &H, intel: bool) -> Result<(), UfsError> {
    let mut tries = 0;
    loop {
        if intel {
            lcc_off(hw)?;
        }
        match uic::run(hw, DME_LINKSTARTUP, [0, 0, 0]) {
            Ok(_) => break,
            Err(e) if tries == RETRIES => return Err(e),
            Err(_) => {
                tries += 1;
                enable::restart(hw)?;
            }
        }
    }
    if hw.r32(HCS) & HCS_DP == 0 {
        return Err(UfsError::NoDevice);
    }
    // The startup raised a LINERESET; reading the register clears it.
    let _ = hw.r32(UECPA);
    Ok(())
}

/// ufs_intel_disable_lcc: written only if set, as Linux does.
fn lcc_off<H: Hw>(hw: &H) -> Result<(), UfsError> {
    if dme::get(hw, PA_LOCAL_TX_LCC_ENABLE)? != 0 {
        dme::set(hw, PA_LOCAL_TX_LCC_ENABLE, 0)?;
    }
    Ok(())
}
