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
use super::state::{dist_base, set_bases};

/// Bring up the distributor and the boot CPU's half of the GIC.
pub fn init_gic(dist: u64, redist: u64) -> Result<(), &'static str> {
    set_bases(dist, redist);
    Gic::new(dist).init()
}

/// Enable `irq`. An SGI or PPI is enabled for the calling CPU only.
pub fn enable_irq(irq: u32) {
    Gic::new(dist_base()).enable_irq(irq);
}

pub fn disable_irq(irq: u32) {
    Gic::new(dist_base()).disable_irq(irq);
}

pub fn send_sgi(target: u32, intid: u32) -> Result<(), ()> {
    super::icc::send_sgi(target, intid)
}

pub fn send_sgi_all_others(intid: u32) -> Result<(), ()> {
    super::icc::send_sgi_all_others(intid)
}
