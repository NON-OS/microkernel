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

//! One bring-up attempt on the host discovery found. A failed one stops the
//! host and waits for it to stop before anything goes back, so no list is
//! still running when the regions it reads are unmapped. A host that never
//! stops keeps its regions: only the claim goes back, whose release stops
//! bus mastering, and the regions are leaked rather than handed on.

use super::discover::Found;
use super::env::HostEnv;
use super::held::Held;
use super::hosts::INTEL;
use super::open::take;
use crate::ufs::bring_up::bring_up;
use crate::ufs::disk::Ufs;
use crate::ufs::hci::enable::stop;
use crate::ufs::utp::layout::Lists;

/// The host up and its units found, with what it holds from the broker.
pub struct Opened {
    pub env: HostEnv,
    pub ufs: Ufs,
    _held: Held,
}

pub fn attempt(dev: &Found) -> Result<Opened, &'static str> {
    let (env, held) = take(dev).ok_or("the broker refused a grant")?;
    let r = &held.regions;
    let lists = Lists { utrl: r[0].buf, utmrl: r[1].buf, ucd: r[2].buf, data: r[3].buf };
    match bring_up(&env, lists, dev.vendor == INTEL) {
        Ok(ufs) => Ok(Opened { env, ufs, _held: held }),
        Err(_) => {
            if !stop(&env) {
                held.leak_regions();
                return Err("bring-up failed; host would not stop, regions kept");
            }
            Err("bring-up failed")
        }
    }
}
