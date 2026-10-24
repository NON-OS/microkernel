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

//! The order of the power-off. FLUSH CACHE EXT comes first and STANDBY
//! IMMEDIATE runs whatever the flush did: a disk spun down by the drive's
//! own standby still writes back what it can, and an emergency power loss
//! with the heads loaded is what this is here to prevent. Pure, so the host
//! proofs hold it.

use crate::error::AhciResult;

/// How each of the two commands ended.
pub struct PoweredOff {
    pub flushed: AhciResult<()>,
    pub standby: AhciResult<()>,
}

/// Run `flush` on `port`, then `standby` whatever `flush` returned.
pub fn power_off<P, F, S>(port: &mut P, flush: F, standby: S) -> PoweredOff
where
    F: FnOnce(&mut P) -> AhciResult<()>,
    S: FnOnce(&mut P) -> AhciResult<()>,
{
    let flushed = flush(port);
    PoweredOff { flushed, standby: standby(port) }
}

impl PoweredOff {
    /// The first failure, the flush's before the standby's.
    pub fn failure(&self) -> AhciResult<()> {
        self.flushed.and(self.standby)
    }
}
