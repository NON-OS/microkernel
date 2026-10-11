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

//! ufshcd_hba_stop and ufshcd_hba_execute_hce: HCE to 0 and seen 0, then to
//! 1, and the controller's own initialisation seen through by HCE reading 1
//! again. It goes 1, 0, 1 while the host initialises, so it is not read
//! until a millisecond has passed, as Linux does.

use super::regs::{HCE, HCE_ENABLE};
use super::wait::until;
use crate::ufs::env::{pause, Hw};
use crate::ufs::error::UfsError;
use crate::ufs::text::Line;
use crate::ufs::time::{HCE_OFF_MS, HCE_ON_MS};

pub const HCE_STUCK_ON: &str = "host would not stop: HCE still 1 after 10 ms";
pub const HCE_STUCK_OFF: &str = "host would not start: HCE not 1 after 56 ms";

/// HCE to 0, and true once it reads 0 within HCE_OFF_MS: the host has
/// stopped and masters nothing more.
pub fn stop<H: Hw>(hw: &H) -> bool {
    hw.w32(HCE, 0);
    until(hw, HCE_OFF_MS, || hw.r32(HCE) & HCE_ENABLE == 0)
}

/// Reset the host and bring it back; the UniPro link is down afterwards.
pub fn restart<H: Hw>(hw: &H) -> Result<(), UfsError> {
    if !stop(hw) {
        return Err(failed(hw, HCE_STUCK_ON));
    }
    hw.w32(HCE, HCE_ENABLE);
    pause(hw, 1);
    if !until(hw, HCE_ON_MS - 1, || hw.r32(HCE) & HCE_ENABLE != 0) {
        return Err(failed(hw, HCE_STUCK_OFF));
    }
    Ok(())
}

fn failed<H: Hw>(hw: &H, what: &str) -> UfsError {
    hw.line(Line::new().s(what.as_bytes()).as_bytes());
    UfsError::Host
}
