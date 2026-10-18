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

//! What a host is made from, kept so a removable slot can make one again
//! for each card that comes.

use super::super::env::{Clock, DmaBuf, Log, Mmio};
use super::super::sdhci::Host;

#[derive(Clone)]
pub struct HostParts<M, C, L> {
    pub io: M,
    pub clock: C,
    pub log: L,
    /// The page the ADMA2 descriptor table is written to.
    pub table: DmaBuf,
    pub intel_emmc: bool,
}

impl<M: Mmio + Clone, C: Clock + Clone, L: Log + Clone> HostParts<M, C, L> {
    pub fn host(&self) -> Host<M, C, L> {
        Host::new(
            self.io.clone(),
            self.clock.clone(),
            self.log.clone(),
            self.table,
            self.intel_emmc,
        )
    }
}
