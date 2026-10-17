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

//! An SPI aimed at the calling CPU, whichever GIC this is.

use super::distributor::GicDistributor;
use super::intid::LINES;
use super::state::{dist_base, version};
use super::v2::of_affinity;
use super::version::GicVersion;
use crate::arch::aarch64::cpu::{cpu_affinity, mpidr_affinity};

/// Deliver SPI `intid` to this CPU alone: GICD_IROUTER with this CPU's
/// affinity on a GICv3, its one-hot interface number in GICD_ITARGETSR on
/// a GICv2.
pub fn route_spi_here(intid: u32) -> Result<(), &'static str> {
    if !(32..LINES).contains(&intid) {
        return Err("not an SPI");
    }
    let distributor = GicDistributor::new(dist_base());
    match version() {
        Some(GicVersion::V3) => distributor.set_route(intid, mpidr_affinity()),
        Some(GicVersion::V2) => {
            let mask = of_affinity(cpu_affinity()).ok_or("this CPU has no GICv2 interface")?;
            distributor.set_target(intid, mask);
        }
        None => return Err("no GIC is up"),
    }
    Ok(())
}
