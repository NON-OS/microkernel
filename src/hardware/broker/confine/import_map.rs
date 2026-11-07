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

//! An imported surface's runs into its importer's domain, read only, at one
//! IOVA run (dma/import). Never a physical address: a device no domain
//! confines is refused, whatever the posture.

use super::iova;
use super::map::NoAddress;
use super::table::CAPSULES;
use crate::hardware::broker::dma::import::rule::{DEVICE_READS, DEVICE_WRITES};
use crate::hardware::broker::dma::import::runs::{map_runs, Mapped, Space, Step};
use crate::memory::addr::PhysAddr;
use crate::memory::iommu::{IommuDomain, IommuError, IommuProtection};

struct InDomain<'a>(&'a IommuDomain);

impl Space for InDomain<'_> {
    fn map(&mut self, iova: u64, phys: u64, len: u64) -> Step {
        // SAFETY: eK@nonos.systems - `[phys, phys+len)` is a run of a surface's
        // frames, which the importer's attach keeps allocated until the import
        // is taken out (dma/import drops it before the attach goes), and which
        // are quarantined if that unmap fails. `[iova, iova+len)` is fresh from
        // this domain's allocator; both are page aligned. The device reads only.
        let access = IommuProtection { read: DEVICE_READS, write: DEVICE_WRITES, mmio: false };
        match unsafe { self.0.map(iova, PhysAddr::new(phys), len as usize, access) } {
            Ok(()) => Step::Mapped,
            Err(IommuError::Uncommitted) => Step::Uncommitted,
            Err(IommuError::Stranded) => Step::Stranded,
            Err(_) => Step::Refused,
        }
    }

    fn unmap(&mut self, iova: u64, len: u64) -> bool {
        self.0.unmap(iova, len as usize).is_ok()
    }
}

/// The IOVA `runs` (`length` bytes in all) now sit at in the domain of the
/// capsule `pid` that holds `device_id`.
pub(in crate::hardware::broker) fn map_import(
    pid: u32,
    device_id: u64,
    runs: &[(u64, u64)],
    length: u64,
) -> Result<u64, NoAddress> {
    let mut all = CAPSULES.lock();
    let held = |c: &&mut super::table::Capsule| c.devices.iter().any(|(d, _)| *d == device_id);
    let Some(c) = all.iter_mut().filter(|c| c.pid == pid).find(held) else {
        return Err(NoAddress::Unconfined);
    };
    let at = iova::take(c, length).ok_or(NoAddress::Failed)?;
    match map_runs(&mut InDomain(&c.domain), at, runs) {
        Mapped::All => Ok(at),
        Mapped::Undone => {
            iova::give_back(c, at, length);
            Err(NoAddress::Failed)
        }
        Mapped::Stranded => Err(NoAddress::Stranded),
    }
}
