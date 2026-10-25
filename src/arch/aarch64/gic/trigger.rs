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

use super::distributor::GicDistributor;
use super::enable::disable_irq;
use super::state::{dist_base, version};

/// Set an SPI's trigger. The line is disabled first: changing a trigger
/// while the interrupt is enabled is UNPREDICTABLE, so the caller enables
/// it again once its handler is in place. SGIs and PPIs are the kernel's
/// and are not configured here.
pub fn configure_spi(intid: u32, edge: bool) -> Result<(), &'static str> {
    if !(32..=1019).contains(&intid) {
        return Err("not an SPI");
    }
    if version().is_none() {
        return Err("no GIC in service");
    }
    disable_irq(intid);
    // GICD_ICFGR holds SPI triggers at the same offset on GICv2 and v3.
    GicDistributor::new(dist_base()).set_config(intid, edge);
    Ok(())
}
