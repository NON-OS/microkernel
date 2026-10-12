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

//! The platform devices in _DEP order: each after the devices it depends
//! on, so a driver claiming one finds what it waits on already in the
//! table. A device whose dependency is missing or dangling is still
//! published, once, flagged; one in a cycle likewise, after the rest. A
//! dependency that is not a device (a power resource, a region) counts as
//! present.

use crate::boot::evaluator;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::deps::{dependencies, order, Verdict};
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::platform::{DeviceInfo, PRESENT};
use nonos_aml::platform_device::PlatformDevice;

/// Indices into `found`, in publishing order, each with its verdict.
pub fn ordered(
    devices: &[DeviceInfo],
    aml: &mut Aml,
    host: &mut CapsuleHost,
    found: &[PlatformDevice],
) -> Vec<(usize, Verdict)> {
    let mut eval = evaluator(host);
    let deps: Vec<_> = found.iter().map(|d| (d.id, dependencies(aml, &mut eval, d.id))).collect();
    let present = |id: Id| devices.iter().find(|d| d.id == id).is_none_or(|d| d.sta & PRESENT != 0);
    let placed = order(&deps, &present);
    let at = |id: Id| found.iter().position(|d| d.id == id);
    placed.iter().filter_map(|p| at(p.id).map(|i| (i, p.verdict))).collect()
}

/// Before the device's own line: why it goes to the kernel flagged.
pub fn deps_unmet(path: &[u8], v: Verdict) {
    let why: &[u8] = match v {
        Verdict::Cycle => b"its _DEP is in a cycle",
        _ => b"a _DEP entry is missing or names nothing",
    };
    Line::new(b"platform ").str(path).str(b": published flagged, ").str(why).end();
}
