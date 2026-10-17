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

//! The capsule's domain a requester goes into, made on its first device.

extern crate alloc;

use alloc::vec::Vec;

use super::attach::Refused;
use super::posture::unconfined_allowed;
use super::say::say;
use super::table::Capsule;
use crate::memory::iommu::{DeviceAddress, IommuDomain};

/// The index of `pid`'s entry in `all`, made with a fresh domain if it has
/// none; `None` when the requester goes ahead unconfined (no unit in
/// service, and not `must`).
pub(super) fn capsule_for(
    all: &mut Vec<Capsule>,
    pid: u32,
    address: DeviceAddress,
    must: bool,
) -> Result<Option<usize>, Refused> {
    Ok(Some(match all.iter().position(|c| c.pid == pid) {
        Some(i) => i,
        None => match IommuDomain::allocate() {
            Ok(domain) => {
                all.push(Capsule::new(pid, domain));
                all.len() - 1
            }
            /*
             * The posture on most hardware: said per claim, not only at boot.
             * No unit in service is the same whether none was found, none is
             * up yet, or the one found (AMD-Vi) has no backend here.
             */
            Err(e) if unconfined_allowed(e) && !must => {
                say(b"unconfined: no remapping unit in service, reaches all memory", pid, address);
                return Ok(None);
            }
            Err(e) if unconfined_allowed(e) => {
                say(b"refused: its IOMMU is not in service to confine it", pid, address);
                return Err(Refused);
            }
            Err(_) => {
                say(b"refused: no domain left", pid, address);
                return Err(Refused);
            }
        },
    }))
}
