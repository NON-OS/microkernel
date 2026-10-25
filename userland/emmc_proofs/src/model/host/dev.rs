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

//! The card in the slot: the SD card when one is there, else the eMMC.

use super::super::dev::Dev;
use super::sim::Sim;

impl Sim {
    pub(super) fn dev(&mut self) -> &mut dyn Dev {
        match &mut self.sd {
            Some(sd) => sd,
            None => &mut self.card,
        }
    }

    pub(super) fn dev_ref(&self) -> &dyn Dev {
        match &self.sd {
            Some(sd) => sd,
            None => &self.card,
        }
    }
}
