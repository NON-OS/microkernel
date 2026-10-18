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

//! A model with the four regions a bring-up is handed, sized as the
//! capsule's.

use super::cfg::Cfg;
use super::mem::Region;
use super::sim::Model;
use crate::ufs::bring_up::bring_up;
use crate::ufs::disk::Ufs;
use crate::ufs::error::UfsError;
use crate::ufs::slot::layout::Lists;

pub struct Rig {
    pub m: Model,
    regions: [Region; 4],
}

impl Rig {
    pub fn new(cfg: Cfg) -> Self {
        let regions = [4096, 4096, 4096, 64 * 1024].map(Region::new);
        Self { m: Model::new(cfg), regions }
    }

    pub fn lists(&self) -> Lists {
        let r = &self.regions;
        Lists { utrl: r[0].buf(), utmrl: r[1].buf(), ucd: r[2].buf(), data: r[3].buf() }
    }

    pub fn up(&self, intel: bool) -> Result<Ufs, UfsError> {
        bring_up(&self.m, self.lists(), intel)
    }
}
