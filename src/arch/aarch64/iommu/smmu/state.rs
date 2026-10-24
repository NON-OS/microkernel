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

//! The driver's one unit, behind one lock. The broker takes its own lock
//! first and never the other way round; nothing here calls back out.

use spin::{Mutex, Once};

use crate::arch::fdt::find::smmu::{IommuMap, SmmuInfo};
use crate::memory::iommu::{DeviceAddress, IommuError};

use super::cmdq::Cmdq;
use super::domain::{Domain, MAX_DOMAINS};
use super::encode::idr::Features;
use super::eventq::Eventq;
use super::mmio::Regs;
use super::strtab::StreamTable;

pub(super) struct Smmu {
    pub(super) regs: Regs,
    pub(super) cmdq: Cmdq,
    pub(super) strtab: StreamTable,
    pub(super) domains: [Option<Domain>; MAX_DOMAINS],
    pub(super) features: Features,
    pub(super) map: Option<IommuMap>,
    pub(super) phandle: u32,
    pub(super) eventq: Eventq,
}

pub(super) static SMMU: Mutex<Option<Smmu>> = Mutex::new(None);

/// What the device tree said, recorded at boot for bring-up to use once
/// the MMIO mapper exists.
pub(super) static DESCRIBED: Once<(SmmuInfo, Option<IommuMap>)> = Once::new();

/// Run `f` on the unit, or answer `NotInitialized` before it is in service.
/// Faults the unit recorded meanwhile are reported on the way out.
pub(super) fn with<T>(f: impl FnOnce(&mut Smmu) -> Result<T, IommuError>) -> Result<T, IommuError> {
    let mut guard = SMMU.lock();
    let smmu = guard.as_mut().ok_or(IommuError::NotInitialized)?;
    let result = f(smmu);
    smmu.eventq.drain(smmu.regs);
    super::gerror::report(smmu.regs);
    result
}

/// What `iommus` entries name the unit by, once it is in service.
pub fn in_service_phandle() -> Option<u32> {
    SMMU.lock().as_ref().map(|smmu| smmu.phandle).filter(|p| *p != 0)
}

impl Smmu {
    /// The stream `device` reaches this unit as, or `None` when nothing
    /// sends it here: a PCI function outside segment 0 or the tree's
    /// `iommu-map`, or a stream past what the table covers.
    pub(super) fn stream(&self, device: DeviceAddress) -> Option<u32> {
        let sid = match device {
            DeviceAddress::Stream(sid) => sid,
            DeviceAddress::Pci { segment: 0, rid } => self.map?.stream_id(rid, self.phandle)?,
            DeviceAddress::Pci { .. } => return None,
        };
        self.strtab.covers(sid).then_some(sid)
    }
}
