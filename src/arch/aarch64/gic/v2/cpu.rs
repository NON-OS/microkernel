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

use super::cpu_interface::CpuInterface;
use super::masks;
use crate::arch::aarch64::boot::multicore::roster;
use crate::arch::aarch64::gic::distributor::GicDistributor;

/// The calling CPU's half of a GICv2: its banked SGI and PPI state, its
/// interface number for SGIs to find it by, and its CPU interface.
pub(in crate::arch::aarch64::gic) fn init_cpu(dist: u64, cpu: u64) -> Result<(), &'static str> {
    let distributor = GicDistributor::new(dist);
    distributor.init_banked_v2();
    let mask = distributor.this_cpu_mask();
    // Only a GIC built for one CPU may leave the number out.
    if mask == 0 && roster::len() > 1 {
        return Err("GICv2 gives this CPU no interface number, so no SGI can reach it");
    }
    masks::record(mask);
    CpuInterface::new(cpu).enable();
    Ok(())
}

/// The distributor and then the boot CPU's half, with every SPI aimed at the
/// boot CPU until someone routes it elsewhere.
pub(in crate::arch::aarch64::gic) fn init(dist: u64, cpu: u64) -> Result<(), &'static str> {
    let distributor = GicDistributor::new(dist);
    distributor.init_v2(distributor.this_cpu_mask());
    init_cpu(dist, cpu)
}
