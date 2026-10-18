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

use super::device::Gic;
use super::distributor::GicDistributor;
use super::intid::LINES;
use super::state::{dist_base, version};
use super::version::GicVersion;

/// Enable `irq`. An SGI or PPI is enabled for the calling CPU only. An LPI
/// is not a line: its enable bit is in the configuration table the ITS
/// driver keeps, and a distributor register past the SPIs is not its.
pub fn enable_irq(irq: u32) {
    if irq >= LINES {
        return;
    }
    match version() {
        Some(GicVersion::V3) => Gic::new(dist_base()).enable_irq(irq),
        Some(GicVersion::V2) => GicDistributor::new(dist_base()).enable_irq(irq),
        None => {}
    }
}

pub fn disable_irq(irq: u32) {
    if irq >= LINES {
        return;
    }
    match version() {
        Some(GicVersion::V3) => Gic::new(dist_base()).disable_irq(irq),
        Some(GicVersion::V2) => GicDistributor::new(dist_base()).disable_irq(irq),
        None => {}
    }
}
