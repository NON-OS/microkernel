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

//! Bringing the UniPro link up. A startup that fails can leave the host's
//! own UniPro stack needing a reset, so each further attempt follows a host
//! restart. Once up, a device must be on the link (HCS.DP).

use super::hce::restart;
use super::uic::{exchange, UicCmd};
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::facts::bounds::LINK_RETRIES;
use crate::ufs::facts::hci::{HCS, HCS_DP, UECPA};
use crate::ufs::facts::uic::PA_LOCAL_TX_LCC_ENABLE;

pub fn up<H: Hw>(hw: &H, intel: bool) -> Result<(), UfsError> {
    let mut attempt = 0;
    while let Err(e) = startup(hw, intel) {
        if attempt == LINK_RETRIES {
            return Err(e);
        }
        attempt += 1;
        restart(hw)?;
    }
    if hw.r32(HCS) & HCS_DP == 0 {
        return Err(UfsError::NoDevice);
    }
    // Startup itself raises a line reset in UECPA; reading clears it, so a
    // later read means something new.
    let _ = hw.r32(UECPA);
    Ok(())
}

fn startup<H: Hw>(hw: &H, intel: bool) -> Result<(), UfsError> {
    if intel {
        lcc_off(hw)?;
    }
    exchange(hw, UicCmd::link_startup()).map(|_| ())
}

/// Written only when set: a DME_SET is a UIC round trip, and most startups
/// find it already clear.
fn lcc_off<H: Hw>(hw: &H) -> Result<(), UfsError> {
    if exchange(hw, UicCmd::get(PA_LOCAL_TX_LCC_ENABLE))? != 0 {
        exchange(hw, UicCmd::set(PA_LOCAL_TX_LCC_ENABLE, 0))?;
    }
    Ok(())
}
