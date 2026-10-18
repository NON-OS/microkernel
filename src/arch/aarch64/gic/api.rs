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
use super::select::select;
use super::state::{cpu_base, dist_base, set_layout, version};
use super::version::GicVersion;
use super::{announce, sysregs, v2};

/// Choose the GIC (see `select`), then bring up its distributor and the boot
/// CPU's half. `cpu` is GICv2's CPU interface and `redist` GICv3's
/// redistributor region; each is ignored by the other version.
pub fn init_gic(
    described: Option<GicVersion>,
    dist: u64,
    cpu: u64,
    redist: u64,
) -> Result<GicVersion, &'static str> {
    let version = select(described, sysregs::present())?;
    set_layout(version, dist, cpu, redist);
    announce::chosen(version, dist, cpu, redist);
    match version {
        GicVersion::V3 => Gic::new(dist).init()?,
        GicVersion::V2 => v2::init(dist, cpu)?,
    }
    Ok(version)
}

/// The calling CPU's half of the GIC, for secondaries once `init_gic` ran.
pub fn init_gic_cpu() -> Result<(), &'static str> {
    match version() {
        Some(GicVersion::V3) => super::cpu::init_gic_cpu(),
        Some(GicVersion::V2) => v2::init_cpu(dist_base(), cpu_base()),
        None => Err("GIC not brought up on the boot CPU"),
    }
}

/// The GIC `init_gic` chose, once it has. A GICv3's CPU interface is system
/// registers in the core, which a core that powers down loses.
pub fn chosen_version() -> Option<GicVersion> {
    version()
}
