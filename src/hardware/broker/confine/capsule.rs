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

use super::decide::verdict;
use super::posture::unconfined_allowed;
use super::refuse::Refused;
use super::say::say;
use super::table::Capsule;
use super::unconfined::unconfined;
use crate::memory::iommu::dev_unconfined::{OPT_IN, PRODUCTION};
use crate::memory::iommu::IommuDomain;

/// The index of `pid`'s entry in `all`, made with a fresh domain if it has
/// none; `None` when the requester goes ahead unconfined, which is the
/// development opt-in's alone (decide.rs, unconfined.rs) and never for
/// `must`, a device the description puts behind a unit.
pub(super) fn capsule_for(
    all: &mut Vec<Capsule>,
    pid: u32,
    device_id: u64,
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
             * No unit in service is the same whether none was found, none is
             * up yet, or the one found (AMD-Vi) has no backend here.
             */
            Err(e) if unconfined_allowed(e) && !must => {
                unconfined(device_id, verdict(true, true, Err(true), OPT_IN, PRODUCTION))?;
                return Ok(None);
            }
            Err(e) if unconfined_allowed(e) => {
                say(b"refused: its IOMMU is not in service to confine it");
                return Err(Refused);
            }
            Err(_) => {
                say(b"refused: no domain left");
                return Err(Refused);
            }
        },
    }))
}
