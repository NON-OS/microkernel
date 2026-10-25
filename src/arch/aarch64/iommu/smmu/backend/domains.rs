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

use crate::memory::iommu::{DomainId, IommuError};

use super::super::domain::{asid, id, slot, Domain, MAX_STREAMS};
use super::super::encode::{cd, cmd, ste};
use super::super::state::with;
use super::super::{frame, walk};

/// A fresh domain: an empty level 1 table and the CD over it, written
/// before any STE can point at it.
pub fn allocate_domain() -> Result<DomainId, IommuError> {
    with(|smmu| {
        let free = smmu.domains.iter().position(Option::is_none);
        let at = free.ok_or(IommuError::DomainExhausted)?;
        let ttb = frame::alloc()?;
        let cd_frame = match frame::alloc() {
            Ok(f) => f,
            Err(e) => {
                frame::free(ttb);
                return Err(e);
            }
        };
        let words = cd::cd(ttb, asid(at), smmu.features.oas_code);
        frame::words(cd_frame)?[..8].copy_from_slice(&words);
        super::super::wait::publish();
        smmu.domains[at] = Some(Domain { ttb, cd: cd_frame, streams: [0; MAX_STREAMS], count: 0 });
        Ok(id(at))
    })
}

/// Every stream still in the domain goes back to abort, its TLB entries
/// go, and only then are its frames returned. If the unit fails any step
/// the frames stay allocated, never reused while it might walk them.
pub fn free_domain(domain: DomainId) -> Result<(), IommuError> {
    with(|smmu| {
        let at = slot(domain).ok_or(IommuError::InvalidDomain)?;
        let d = smmu.domains[at].ok_or(IommuError::InvalidDomain)?;
        for &sid in &d.streams[..d.count] {
            smmu.strtab.write(sid, ste::abort(), &mut smmu.cmdq, smmu.regs)?;
        }
        smmu.cmdq.run(smmu.regs, &[cmd::tlbi_nh_asid(asid(at))])?;
        smmu.domains[at] = None;
        walk::free_tree(d.ttb);
        frame::free(d.cd);
        Ok(())
    })
}
