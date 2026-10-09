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
//! What the survey found on this machine.

use super::found::Found;
use crate::controller::verdict::Verdict;

pub(super) const MAX_CONTROLLERS: usize = 4;

#[derive(Clone, Copy)]
pub struct Survey {
    pub hda: [Option<Found>; MAX_CONTROLLERS],
    pub amd_acp: bool,
    /// The device id of an Intel SST engine on the bus (`controller::sst`).
    pub intel_sst: Option<u16>,
}

impl Survey {
    pub fn controllers(&self) -> impl Iterator<Item = &Found> {
        self.hda.iter().flatten()
    }

    pub fn is_empty(&self) -> bool {
        self.hda.iter().all(|f| f.is_none())
    }

    /// What the bus alone says before any controller is tried. An SST
    /// engine means the speakers are behind Intel's DSP; an HD Audio
    /// controller beside it (Broadwell's display audio) carries HDMI only.
    pub fn bus_verdict(&self) -> Option<Verdict> {
        self.intel_sst.map(|_| Verdict::NeedsSof)
    }
}
