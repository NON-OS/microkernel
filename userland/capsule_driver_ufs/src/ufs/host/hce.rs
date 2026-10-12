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

//! The host enable bit. Writing HCE 0 stops the host and drops the link;
//! writing 1 starts the host's own initialisation, which is through when
//! HCE reads 1 again. A host is always stopped before it is started, so it
//! starts from a known state whatever firmware left.

use super::wait::until;
use crate::ufs::env::{pause, Hw};
use crate::ufs::error::UfsError;
use crate::ufs::facts::bounds::{HCE_FIRST_READ_MS, HCE_OFF_MS, HCE_ON_MS};
use crate::ufs::facts::hci::{HCE, HCE_ENABLE};
use crate::ufs::text::Line;

/// Quote HCE_OFF_MS and HCE_ON_MS; ufs_proofs holds them equal.
pub const HCE_STUCK_ON: &str = "host would not stop: HCE still 1 after 10 ms";
pub const HCE_STUCK_OFF: &str = "host would not start: HCE not 1 after 100 ms";

/// HCE to 0; true once it reads 0, when the host masters nothing more.
pub fn stop<H: Hw>(hw: &H) -> bool {
    hw.w32(HCE, 0);
    until(hw, HCE_OFF_MS, || hw.r32(HCE) & HCE_ENABLE == 0)
}

/// Stopped, then started and seen through its initialisation.
pub fn restart<H: Hw>(hw: &H) -> Result<(), UfsError> {
    if !stop(hw) {
        return Err(said(hw, HCE_STUCK_ON));
    }
    hw.w32(HCE, HCE_ENABLE);
    pause(hw, HCE_FIRST_READ_MS);
    let rest = HCE_ON_MS - HCE_FIRST_READ_MS;
    if !until(hw, rest, || hw.r32(HCE) & HCE_ENABLE != 0) {
        return Err(said(hw, HCE_STUCK_OFF));
    }
    Ok(())
}

fn said<H: Hw>(hw: &H, what: &str) -> UfsError {
    hw.line(Line::new().s(what.as_bytes()).as_bytes());
    UfsError::Host
}
